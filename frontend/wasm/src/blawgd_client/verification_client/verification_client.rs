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
    pub async fn get_profile_info(
        &mut self,
        address: String,
    ) -> Result<(AccountInfo, FollowingCount)> {
        let lb = self
            .lc
            .supervisor()
            .latest_trusted()
            .ok_or(anyhow!("could not fetch light block from the light client"))?;

        let resp = query_client::QueryClient::new(self.client.clone())
            .get_profile_info(GetProfileInfoRequest {
                address: address.clone(),
                height: lb.signed_header.header.height.value() as i64 - 1,
            })
            .await?;
        let resp = resp.into_inner();

        let account_into_map = resp.account_info.clone();
        let account_info_key = account_into_map
            .keys()
            .next()
            .ok_or(anyhow!("account info key does not exist"))?;
        let account_info = account_into_map.get(account_info_key);
        let following_count_map = resp.following_count.clone();
        let following_count_key = following_count_map
            .keys()
            .next()
            .ok_or(anyhow!("following count key does not exist"))?;
        let following_count = following_count_map.get(account_info_key);
        let root = lb.signed_header.header.app_hash.value();

        let mut proof: tendermint_proto::crypto::ProofOps = prost::Message::decode(
            resp.proofs
                .get(account_info_key)
                .ok_or(anyhow!("could not get proof for account info"))?
                .as_slice(),
        )?;
        let mut proof = convert_tm_to_ics_merkle_proof(proof)?;

        let mut value: Vec<u8> = Vec::new();
        if account_info.is_some() {
            prost::Message::encode(account_info.unwrap(), &mut value);
        }
        if !value.is_empty() {
            verify_membership(
                proof,
                root.as_ref(),
                account_info_key.as_bytes(),
                value.as_ref(),
            )
            .context("failed to verify membership of account info")?;
        } else {
            verify_non_membership(proof, root.as_ref(), account_info_key.as_bytes())
                .context("failed to verify non member ship of account info")?;
        };

        let mut proof: tendermint_proto::crypto::ProofOps = prost::Message::decode(
            resp.proofs
                .get(following_count_key)
                .ok_or(anyhow!("could not get proof for account info"))?
                .as_slice(),
        )?;
        let mut proof = convert_tm_to_ics_merkle_proof(proof)?;

        let mut value: Vec<u8> = Vec::new();
        if following_count.is_some() {
            prost::Message::encode(following_count.unwrap(), &mut value);
        }
        if !value.is_empty() {
            verify_membership(
                proof,
                root.as_ref(),
                following_count_key.as_bytes(),
                value.as_ref(),
            )
            .context("failed to verify membership of account info")?;
        } else {
            verify_non_membership(proof, root.as_ref(), following_count_key.as_bytes())
                .context("failed to verify non member ship of account info")?;
        };

        let account_info = account_info
            .unwrap_or(&AccountInfo {
                address: "".to_string(),
                name: "".to_string(),
                photo: "".to_string(),
                metadata: "".to_string(),
            })
            .clone();
        let account_info = crate::util::normalize_account_info(account_info, address.clone());

        let mut following_count = following_count
            .unwrap_or(&FollowingCount {
                address: address.clone(),
                count: 0,
            })
            .clone();
        following_count.address = address.clone();

        Ok((account_info, following_count))
    }
}
