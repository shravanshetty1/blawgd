use super::keys;
use super::VerificationClient;
use crate::blawgd_client::verification_client::helpers::{
    convert_tm_to_ics_merkle_proof, get_exist_proof,
};
use crate::blawgd_client::verification_client::proof::{verify_membership, verify_non_membership};
use crate::blawgd_client::{query_client, AccountInfo, GetRequest, Post, PostView};
use anyhow::bail;
use anyhow::ensure;
use anyhow::Result;
use anyhow::{anyhow, Context};
use hex;
use prost::DecodeError;
use std::convert::TryInto;
use std::error::Error;
use tendermint::merkle::proof;
use tendermint_light_client::supervisor::Handle;
use wasm_bindgen::__rt::std::collections::HashMap;

impl VerificationClient {
    pub async fn get(&self, keys: Vec<String>) -> Result<HashMap<String, Vec<u8>>> {
        let lb = self
            .lc
            .latest_trusted()
            .await?
            .ok_or(anyhow!("could not get latest trusted light block"))?;
        let height = lb.signed_header.header.height.value() - 1;
        let root = lb.signed_header.header.app_hash.value();

        let resp = query_client::QueryClient::new(self.client.clone())
            .get(GetRequest {
                height,
                keys: keys.clone(),
            })
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

    pub async fn get_account_info(&self, address: String) -> Result<AccountInfo> {
        let account_info_key = keys::account_info_key(address.clone());

        let keys = vec![account_info_key.clone()];
        let resp = self.get(keys).await?;

        let account_info_raw = resp
            .get(&account_info_key)
            .ok_or(anyhow!("unexpected! did not get account info"))?
            .clone();
        let account_info: AccountInfo = prost::Message::decode(account_info_raw.as_slice())?;
        let account_info = normalize_account_info(account_info, address.clone());

        Ok(account_info)
    }

    pub async fn get_following_list(&self, address: String) -> Result<Vec<String>> {
        let following_list_key = keys::following_key(address);

        let keys = vec![following_list_key.clone()];
        let resp = self.get(keys).await?;

        let following_list_bytes = resp
            .get(&following_list_key)
            .ok_or(anyhow!(
                "unexpected! did not get data for key {}",
                following_list_key
            ))?
            .clone();

        let following_list_raw = String::from_utf8(following_list_bytes)?;
        let following_list: Vec<&str> = following_list_raw.split(",").collect();
        let following_list: Vec<String> = following_list
            .iter()
            .map(|s| String::from(s.clone()))
            .collect();

        Ok(following_list)
    }
    pub async fn get_post_by_account(&self, address: String) -> Result<Vec<PostView>> {
        Ok(Vec::new())
    }
    pub async fn get_post_by_parent_post(&self, parent_post: String) -> Result<Vec<PostView>> {
        Ok(Vec::new())
    }

    pub async fn get_post(&self, id: String) -> Result<PostView> {
        Ok(PostView {
            id,
            creator: None,
            content: "".to_string(),
            parent_post: "".to_string(),
            comments_count: 0,
        })
    }
}

pub fn normalize_account_info(mut account_info: AccountInfo, address: String) -> AccountInfo {
    account_info.address = address.clone();
    if account_info.photo.is_empty() {
        account_info.photo = "/profile.jpeg".into();
    }
    if account_info.name.is_empty() {
        let name_suffix: String = address.chars().skip(address.len() - 5).take(5).collect();
        account_info.name = format!("anon{}", name_suffix);
    }
    account_info
}
