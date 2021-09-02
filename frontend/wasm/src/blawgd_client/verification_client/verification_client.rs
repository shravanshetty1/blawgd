use super::keys;
use super::VerificationClient;
use crate::blawgd_client::verification_client::helpers::{
    convert_tm_to_ics_merkle_proof, get_exist_proof,
};
use crate::blawgd_client::verification_client::proof::{verify_membership, verify_non_membership};
use crate::blawgd_client::{
    query_client, AccountInfo, FollowingCount, GetProfileInfoRequest, GetRequest,
};
use crate::light_client::LightClient;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Result;
use anyhow::{anyhow, Context};
use hex;
use prost::DecodeError;
use std::convert::TryInto;
use std::error::Error;
use tendermint::merkle::proof;
use wasm_bindgen::__rt::std::collections::HashMap;

impl<'a> VerificationClient<'a> {
    pub async fn get(&self, keys: Vec<String>) -> Result<HashMap<String, Vec<u8>>> {
        let lb = self
            .lc
            .supervisor()
            .latest_trusted()
            .ok_or(anyhow!("could not get latest trusted light block"))?;
        let height = lb.signed_header.header.height.value() - 1;
        let root = lb.signed_header.header.app_hash.value();

        let resp = query_client::QueryClient::new(self.client.clone())
            .get(GetRequest { height, keys })
            .await?
            .into_inner();

        let data = resp.data;
        let proofs = resp.proofs;
        for key in keys {
            let val = data
                .get(&key)
                .ok_or(anyhow!("did not get data for key {}", key))?
                .clone();

            let proof = proofs
                .get(&key)
                .ok_or(anyhow!("did not get proof for key {}", key))?
                .clone();
            let mut proof: tendermint_proto::crypto::ProofOps =
                prost::Message::decode(proof.as_slice())?;
            let mut proof = convert_tm_to_ics_merkle_proof(proof)?;

            if val.is_empty() {
                verify_non_membership(proof, root.as_slice(), key.as_bytes())?;
            } else {
                verify_membership(proof, root.as_slice(), key.as_bytes(), val.as_slice())?;
            }
        }
        Ok(data)
    }

    pub async fn get_account_info(&mut self, address: String) -> Result<AccountInfo> {
        let account_info_key = keys::account_info_key(address);

        let keys = vec![account_info_key];
        let resp = self.get(keys).await?;

        let account_info_raw = resp
            .get(&account_info_key)
            .ok_or(anyhow!("unexpected! did not get account info"))?
            .clone();
        let account_info: AccountInfo = prost::Message::decode(account_info_raw.as_slice())?;

        Ok(account_info)
    }
}
