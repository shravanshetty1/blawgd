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
    pub async fn get(&self, keys: Vec<String>) -> Result<HashMap<String, Option<Vec<u8>>>> {
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
        let result: HashMap<String, Option<Vec<u8>>> = data
            .iter()
            .map(|(k, v)| {
                let k = k.clone();
                if v.is_empty() {
                    (k, None)
                } else {
                    (k, Some(v.clone()))
                }
            })
            .collect();
        Ok(result)
    }

    pub async fn get_proto<T: prost::Message + std::default::Default>(
        &self,
        keys: Vec<String>,
    ) -> Result<HashMap<String, Option<T>>> {
        let data = self.get(keys).await?;
        let mut result: HashMap<String, Option<T>> = HashMap::new();
        for (k, v) in data {
            if v.is_some() {
                let v: T = prost::Message::decode(v.unwrap().clone().as_slice())?;
                result.insert(k, Some(v));
            } else {
                result.insert(k, None);
            }
        }
        Ok(result)
    }

    pub async fn get_account_info(&self, address: String) -> Result<AccountInfo> {
        let account_info = self
            .get_proto::<AccountInfo>(vec![keys::account_info_key(address.clone())])
            .await?
            .values()
            .next()
            .cloned()
            .ok_or(anyhow!("could not get account info"))?
            .unwrap_or(AccountInfo {
                address: address.clone(),
                name: "".to_string(),
                photo: "".to_string(),
                following_count: 0,
                followers_count: 0,
                post_count: 0,
            });
        Ok(normalize_account_info(account_info, address.clone()))
    }

    pub async fn get_following_list(&self, address: String) -> Result<Vec<String>> {
        let following_list_key = keys::following_key(address);
        let following_list_raw = self
            .get(vec![following_list_key.clone()])
            .await?
            .get(&following_list_key)
            .ok_or(anyhow!(
                "unexpected! did not get data for key {}",
                following_list_key
            ))?
            .clone();

        if following_list_raw.is_some() {
            Ok(String::from_utf8(following_list_raw.unwrap())?
                .split(",")
                .map(|s| String::from(s.clone()))
                .collect())
        } else {
            Ok(Vec::new())
        }
    }
    pub async fn get_post_by_account(&self, address: String) -> Result<Vec<PostView>> {
        let account_info = self.get_account_info(address.clone()).await?;
        let mut keys: Vec<String> = Vec::new();
        if account_info.post_count == 0 {
            return Ok(Vec::new());
        }
        for i in (1..account_info.post_count + 1).rev() {
            keys.push(keys::user_post_key(address.clone(), i.to_string()))
        }
        let post_ids: Result<Vec<String>, _> = self
            .get(keys)
            .await?
            .values()
            .map(|b| -> Result<String> {
                Ok(String::from_utf8(
                    b.clone().ok_or(anyhow!("could not get a user post"))?,
                )?)
            })
            .collect();
        self.get_posts(post_ids?).await
    }

    pub async fn get_post_by_parent_post(&self, parent_post_id: String) -> Result<Vec<PostView>> {
        let parent_post = self.get_post(parent_post_id.clone()).await;
        if parent_post.is_err() {
            return Ok(Vec::new());
        }
        let parent_post = parent_post?;
        if parent_post.comments_count == 0 {
            return Ok(Vec::new());
        }
        let mut keys: Vec<String> = Vec::new();
        for i in (1..parent_post.comments_count + 1).rev() {
            keys.push(keys::subpost_key(parent_post.id.clone(), i.to_string()))
        }
        let post_ids: Result<Vec<String>, _> = self
            .get(keys)
            .await?
            .values()
            .map(|b| -> Result<String> {
                Ok(String::from_utf8(b.clone().ok_or(anyhow!(
                    "could not get sub post for parent post {}",
                    parent_post_id
                ))?)?)
            })
            .collect();
        self.get_posts(post_ids?).await
    }

    pub async fn get_posts(&self, ids: Vec<String>) -> Result<Vec<PostView>> {
        let keys: Vec<String> = ids.iter().map(|id| keys::post_key(id.clone())).collect();
        let posts: Result<Vec<Post>, _> = self
            .get_proto::<Post>(keys)
            .await?
            .values()
            .map(|p| p.clone().ok_or(anyhow!("could not get post")))
            .collect();
        let posts = posts?;
        let account_infos = self
            .get_proto::<AccountInfo>(
                posts
                    .clone()
                    .iter()
                    .map(|v| keys::account_info_key(v.creator.clone()))
                    .collect(),
            )
            .await?;
        let post_views: Vec<PostView> = posts
            .iter()
            .map(|p| {
                let p = p.clone();
                let account_info = account_infos
                    .get(&keys::account_info_key(p.creator.clone()))
                    .cloned()
                    .unwrap_or(Some(AccountInfo {
                        address: "".to_string(),
                        name: "".to_string(),
                        photo: "".to_string(),
                        following_count: 0,
                        followers_count: 0,
                        post_count: 0,
                    }))
                    .unwrap_or(AccountInfo {
                        address: "".to_string(),
                        name: "".to_string(),
                        photo: "".to_string(),
                        following_count: 0,
                        followers_count: 0,
                        post_count: 0,
                    });
                let account_info = normalize_account_info(account_info, p.creator.clone());
                PostView {
                    id: p.id,
                    creator: Some(account_info),
                    content: p.content,
                    parent_post: p.parent_post,
                    comments_count: p.comments_count,
                }
            })
            .collect();
        Ok(post_views)
    }

    pub async fn get_post(&self, id: String) -> Result<PostView> {
        let post_view = self
            .get_posts(vec![id.clone()])
            .await?
            .first()
            .ok_or(anyhow!("could not get post with id {}", id))?
            .clone();
        Ok(post_view)
    }

    pub async fn get_timeline(&self, address: String) -> Result<Vec<PostView>> {
        let followings = self.get_following_list(address).await?;
        let mut key_to_address: HashMap<String, String> = HashMap::new();
        for addr in followings.clone() {
            key_to_address.insert(keys::account_info_key(addr.clone()), addr.clone());
        }
        let account_info_keys: Vec<String> = followings
            .iter()
            .map(|v| keys::account_info_key(v.clone()))
            .collect();
        let account_infos = self.get_proto::<AccountInfo>(account_info_keys).await?;
        let mut user_post_keys: Vec<String> = Vec::new();
        for (k, info) in account_infos {
            if info.is_none() {
                continue;
            }
            let info = info.unwrap();
            let post_count = info.post_count;
            if post_count == 0 {
                continue;
            }

            for i in 1..post_count + 1 {
                user_post_keys.push(keys::user_post_key(
                    key_to_address
                        .get(&k.clone())
                        .cloned()
                        .ok_or(anyhow!("couldnt find address for key {}", k))?,
                    i.to_string(),
                ))
            }
        }

        let post_keys: Result<Vec<String>, _> = self
            .get(user_post_keys)
            .await?
            .values()
            .map(|v| -> Result<String> {
                Ok(String::from_utf8(
                    v.clone().ok_or(anyhow!("could not get sub post"))?,
                )?)
            })
            .collect();

        self.get_posts(post_keys?).await
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
