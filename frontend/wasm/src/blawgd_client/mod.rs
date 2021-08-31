include!("shravanshetty1.samachar.samachar.rs");
use crate::light_client::LightClient;
use anyhow::anyhow;
use anyhow::ensure;
use anyhow::Result;
use hex;
use prost::DecodeError;
use std::convert::TryInto;
use std::error::Error;
use tendermint::merkle::proof;

// This is a grpc client wrapper which verifies responses using merkle proofs.
// All responses need to be represented in key-pairs such that proofs for keys can be looked up.
// Values are assumed to be stored as protobuf encoded objects and keys are utf8 bytes.
pub struct VerificationClient {
    lc: LightClient,
    client: grpc_web_client::Client,
}

impl VerificationClient {
    pub fn new(lc: LightClient, client: grpc_web_client::Client) -> VerificationClient {
        VerificationClient { lc, client }
    }

    pub async fn get_profile_info(&self, address: String) -> Result<(AccountInfo, FollowingCount)> {
        let lb = self.lc.supervisor().latest_trusted().unwrap();

        let resp = query_client::QueryClient::new(self.client.clone())
            .get_profile_info(GetProfileInfoRequest {
                address: address.clone(),
                height: lb.signed_header.header.height.value() as i64 - 1,
            })
            .await
            .unwrap();
        let resp = resp.into_inner();

        let account_into_map = resp.account_info.clone();
        let account_info_key = account_into_map.keys().next().unwrap();
        let account_info = account_into_map.get(account_info_key);
        let following_count_map = resp.following_count.clone();
        let following_count_key = following_count_map.keys().next().unwrap();
        let following_count = following_count_map.get(account_info_key);

        let mut proof: tendermint_proto::crypto::ProofOps =
            prost::Message::decode(resp.proofs.get(account_info_key).unwrap().as_slice()).unwrap();
        let mut parsed_proof = convert_tm_to_ics_merkle_proof(proof);
        let spec = ics23::iavl_spec();

        let mut valid = false;
        if account_info.is_some() {
            let mut account_info_proto: Vec<u8> = Vec::new();
            prost::Message::encode(account_info.unwrap(), &mut account_info_proto);

            crate::util::console_log("member");

            let parsed_proof = parsed_proof.first().unwrap().clone();
            let e_proof = get_exist_proof(&parsed_proof, account_info_key.as_bytes()).unwrap();
            crate::util::console_log(
                String::from_utf8_lossy(e_proof.key.as_ref())
                    .to_string()
                    .as_str(),
            );
            crate::util::console_log(
                String::from_utf8_lossy(e_proof.value.as_ref())
                    .to_string()
                    .as_str(),
            );
            crate::util::console_log(
                String::from_utf8_lossy(account_info_key.as_bytes())
                    .to_string()
                    .as_str(),
            );
            crate::util::console_log(
                String::from_utf8_lossy(account_info_proto.as_ref())
                    .to_string()
                    .as_str(),
            );
            let root = ics23::calculate_existence_root(e_proof).unwrap();

            // crate::util::console_log(
            //     serde_json::to_string_pretty::<GetProfileInfoResponse>(&resp)
            //         .unwrap()
            //         .as_str(),
            // );
            let x = hex::encode_upper(root);
            crate::util::console_log(x.as_str());
            crate::util::console_log(
                serde_json::to_string_pretty::<tendermint_light_client::types::LightBlock>(&lb)
                    .unwrap()
                    .as_str(),
            );

            valid = ics23::verify_membership(
                &parsed_proof,
                &spec,
                lb.signed_header.header.app_hash.value().as_ref(),
                account_info_key.as_bytes(),
                account_info_proto.as_ref(),
            );
        } else {
            crate::util::console_log("not member");
            valid = ics23::verify_non_membership(
                &parsed_proof.pop().unwrap(),
                &spec,
                lb.signed_header.header.app_hash.value().as_ref(),
                account_info_key.as_bytes(),
            );
        }

        if valid {
            Ok((
                account_info.unwrap().clone(),
                following_count.unwrap().clone(),
            ))
        } else {
            Err(anyhow!("invalid response"))
        }
    }
}

pub fn verify_membership(
    specs: Vec<ics23::ProofSpec>,
    proofs: Vec<ics23::CommitmentProof>,
    root: &[u8],
    key: &[u8],
    value: &[u8],
) -> Result<bool> {
    ensure!(proofs.is_empty(), "proof cannot be empty");
    Ok(true)
}

fn get_exist_proof<'a>(
    proof: &'a ics23::CommitmentProof,
    key: &[u8],
) -> Option<&'a ics23::ExistenceProof> {
    match &proof.proof {
        Some(ics23::commitment_proof::Proof::Exist(ex)) => Some(ex),
        Some(ics23::commitment_proof::Proof::Batch(batch)) => {
            for entry in &batch.entries {
                if let Some(ics23::batch_entry::Proof::Exist(ex)) = &entry.proof {
                    if ex.key == key {
                        return Some(ex);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

pub fn convert_tm_to_ics_merkle_proof(
    tm_proof: tendermint_proto::crypto::ProofOps,
) -> Vec<ics23::CommitmentProof> {
    let mut proofs = vec![];

    for op in &tm_proof.ops {
        let mut parsed = prost_2::Message::decode(op.data.as_slice()).unwrap();
        proofs.push(parsed);
    }

    proofs
}
