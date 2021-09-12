use std::time::Duration;

use async_trait::async_trait;
use contracts::contract_trait;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient as base_client;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;

use tendermint::{abci::transaction::Hash, evidence::Evidence, Time};

use tendermint_light_client::{
    components::{io, io::IoError, scheduler, verifier::ProdVerifier},
    fork_detector::ProdForkDetector,
    light_client,
    operations::ProdHasher,
    predicates::ProdPredicates,
    store::memory::MemoryStore,
    supervisor::Handle,
    types::{LightBlock, PeerId, Status, TrustThreshold},
};

use crate::config;
use crate::util;
use gloo::storage::errors::StorageError;
use gloo::storage::LocalStorage;
use gloo::storage::Storage;
use tendermint::block::Height;
use tendermint_rpc::endpoint::commit;

pub async fn new_supervisor(
    client: grpc_web_client::Client,
) -> tendermint_light_client::supervisor::Supervisor {
    let node_info = base_client::new(client.clone())
        .get_node_info(GetNodeInfoRequest {})
        .await
        .unwrap()
        .get_ref()
        .clone()
        .default_node_info
        .unwrap();
    let peer_id = node_info.default_node_id.parse().unwrap();

    let instance = make_instance(peer_id).await;
    let instance2 = make_instance(peer_id).await;
    let (instances, addresses) = tendermint_light_client::builder::SupervisorBuilder::new()
        .primary(peer_id, config::TENDERMINT_HOST.parse().unwrap(), instance)
        .witness(peer_id, config::TENDERMINT_HOST.parse().unwrap(), instance2)
        .inner();

    let supervisor = tendermint_light_client::supervisor::Supervisor::new(
        instances,
        ProdForkDetector::default(),
        EvidenceReporter,
    );
    supervisor
}

pub async fn start_sync(handler: tendermint_light_client::supervisor::SupervisorHandle) {
    loop {
        match handler.verify_to_highest().await {
            Ok(light_block) => {
                util::console_log(
                    format!("[info] synced to block {}", light_block.height()).as_str(),
                );
            }
            Err(err) => {
                util::console_log(format!("[error] sync failed: {}", err).as_str());
            }
        }
        gloo::timers::future::TimeoutFuture::new(5000).await;
    }
}

async fn make_instance(peer_id: PeerId) -> tendermint_light_client::supervisor::Instance {
    let options = light_client::Options {
        trust_threshold: TrustThreshold::default(),
        // TODO change trusting period
        trusting_period: Duration::from_secs(3600000),
        clock_drift: Duration::from_secs(1),
    };
    let builder = tendermint_light_client::builder::LightClientBuilder::custom(
        peer_id,
        options,
        Box::new(LightStore),
        Box::new(LightClientIO::new(peer_id)),
        Box::new(ProdHasher),
        Box::new(WasmClock),
        Box::new(ProdVerifier::default()),
        Box::new(scheduler::basic_bisecting_schedule),
        Box::new(ProdPredicates),
    );

    let mut trusted_height = util::TRUSTED_HEIGHT.to_string();
    let mut trusted_hash = util::TRUSTED_HASH.to_string();
    if config::ENVIRONMENT == "dev" {
        let resp = get_block(0).await;
        trusted_height = resp.block.header.height.to_string();
        trusted_hash = resp.block_id.hash.to_string();
    }

    let builder = builder
        .trust_primary_at(
            trusted_height.as_str().parse().unwrap(),
            trusted_hash.as_str().parse().unwrap(),
        )
        .await
        .unwrap();

    let instance = builder.build();

    instance
}

fn status_string(s: Status) -> String {
    match s {
        Status::Unverified => "unverified".to_string(),
        Status::Verified => "verified".to_string(),
        Status::Trusted => "trusted".to_string(),
        Status::Failed => "failed".to_string(),
    }
}

fn light_store_key(status: Status, height: Height) -> String {
    format!("light-{}-{}", status_string(status), height)
}

#[derive(Debug)]
pub struct LightStore;

impl tendermint_light_client::store::LightStore for LightStore {
    fn get(&self, height: Height, status: Status) -> Option<LightBlock> {
        LocalStorage::get(light_store_key(status, height)).ok()
    }

    fn update(&mut self, light_block: &LightBlock, status: Status) {
        let height = light_block.signed_header.header.height.clone();
        LocalStorage::set(light_store_key(status, height), light_block);
    }

    fn insert(&mut self, light_block: LightBlock, status: Status) {
        let height = light_block.signed_header.header.height.clone();
        LocalStorage::set(light_store_key(status, height), light_block);
    }

    fn remove(&mut self, height: Height, status: Status) {
        LocalStorage::delete(light_store_key(status, height));
    }

    fn highest(&self, status: Status) -> Option<LightBlock> {
        let local_storage = LocalStorage::raw();
        let length = LocalStorage::length();

        let mut highest: u64 = u64::MIN;
        for i in 0..length {
            let key: String = local_storage.key(i).unwrap().unwrap();
            if !key.starts_with(format!("light-{}-", status_string(status)).as_str()) {
                continue;
            }

            let height: u64 = key
                .strip_prefix(format!("light-{}-", status_string(status)).as_str())
                .unwrap()
                .parse()
                .unwrap();
            if height > highest {
                highest = height;
            }
        }

        self.get(Height::from(highest as u32), status)
    }

    fn lowest(&self, status: Status) -> Option<LightBlock> {
        let local_storage = LocalStorage::raw();
        let length = LocalStorage::length();

        let mut lowest: u64 = u64::MAX;
        for i in 0..length {
            let key: String = local_storage.key(i).unwrap().unwrap();
            if !key.starts_with(format!("light-{}-", status_string(status)).as_str()) {
                continue;
            }

            let height: u64 = key
                .strip_prefix(format!("light-{}-", status_string(status)).as_str())
                .unwrap()
                .parse()
                .unwrap();
            if height < lowest {
                lowest = height;
            }
        }

        self.get(Height::from(lowest as u32), status)
    }

    fn all(&self, status: Status) -> Box<dyn Iterator<Item = LightBlock>> {
        let local_storage = LocalStorage::raw();
        let length = LocalStorage::length();

        let mut lbs = Vec::new();
        for index in 0..length {
            let key: String = local_storage.key(index).unwrap().unwrap();
            if !key.starts_with(format!("light-{}-", status_string(status)).as_str()) {
                continue;
            }

            let lb: Result<LightBlock, StorageError> = LocalStorage::get(key);
            if lb.is_err() {
                continue;
            }
            lbs.push(lb.unwrap());
        }

        Box::new(lbs.into_iter())
    }
}

pub struct WasmClock;

impl tendermint_light_client::components::clock::Clock for WasmClock {
    fn now(&self) -> Time {
        Time::from(chrono::prelude::Utc::now())
    }
}

pub struct EvidenceReporter;

#[contract_trait]
#[async_trait(? Send)]
impl tendermint_light_client::evidence::EvidenceReporter for EvidenceReporter {
    async fn report(&self, e: Evidence, peer: PeerId) -> Result<Hash, IoError> {
        let evidence = serde_json::to_string(&e).unwrap();

        let resp = reqwest::get(
            format!(
                "{}/broadcast_evidence?evidence={}",
                crate::config::TENDERMINT_HOST,
                evidence
            )
            .as_str(),
        )
        .await
        .unwrap()
        .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::evidence::Response>>()
        .await
        .unwrap()
        .into_result()
        .unwrap();

        Ok(resp.hash)
    }
}

pub struct LightClientIO {
    peer_id: PeerId,
}

impl LightClientIO {
    fn new(id: PeerId) -> LightClientIO {
        LightClientIO { peer_id: id }
    }
}

async fn get_block(height: u64) -> tendermint_rpc::endpoint::block::Response {
    let mut param: String = String::new();
    if height != 0 {
        param = format!("?height={}", height)
    }

    reqwest::get(format!("{}/block{}", crate::config::TENDERMINT_HOST, param).as_str())
        .await
        .unwrap()
        .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::block::Response>>()
        .await
        .unwrap()
        .into_result()
        .unwrap()
}

async fn get_commit(height: u64) -> commit::Response {
    let mut param: String = String::new();
    if height != 0 {
        param = format!("?height={}", height)
    }

    reqwest::get(format!("{}/commit{}", crate::config::TENDERMINT_HOST, param).as_str())
        .await
        .unwrap()
        .json::<tendermint_rpc::response::Wrapper<commit::Response>>()
        .await
        .unwrap()
        .into_result()
        .unwrap()
}

#[async_trait(? Send)]
impl io::Io for LightClientIO {
    async fn fetch_light_block(&self, height: io::AtHeight) -> Result<LightBlock, IoError> {
        let height = match height {
            io::AtHeight::At(height) => height.value(),
            io::AtHeight::Highest => 0,
        };

        let signed_header = get_commit(height).await.signed_header;
        let height = signed_header.header.height.value();

        let validator_infos = reqwest::get(
            format!(
                "{}/validators?height={}",
                crate::config::TENDERMINT_HOST,
                height
            )
            .as_str(),
        )
        .await
        .unwrap()
        .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::validators::Response>>()
        .await
        .unwrap()
        .into_result()
        .unwrap()
        .validators;

        let validators = tendermint::validator::Set::with_proposer(
            validator_infos,
            signed_header.header.proposer_address,
        )
        .unwrap();

        let next_validator_infos = reqwest::get(
            format!(
                "{}/validators?height={}",
                crate::config::TENDERMINT_HOST,
                height + 1
            )
            .as_str(),
        )
        .await
        .unwrap()
        .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::validators::Response>>()
        .await
        .unwrap()
        .into_result()
        .unwrap()
        .validators;

        let next_validators = tendermint::validator::Set::without_proposer(next_validator_infos);

        Ok(LightBlock {
            signed_header,
            validators,
            next_validators,
            provider: self.peer_id,
        })
    }
}
