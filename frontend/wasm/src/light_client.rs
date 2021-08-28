use std::time::Duration;

use async_trait::async_trait;
use contracts::contract_trait;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient as base_client;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;

use tendermint::{abci::transaction::Hash, evidence::Evidence, Time};

use tendermint_light_client::{
    components::io, components::io::IoError, components::scheduler,
    components::verifier::ProdVerifier, fork_detector::ProdForkDetector, light_client,
    operations::ProdHasher, predicates::ProdPredicates, store::memory::MemoryStore,
    types::LightBlock, types::PeerId, types::TrustThreshold,
};

use crate::config;
use crate::util;

pub struct LightClient {
    supervisor: tendermint_light_client::supervisor::Supervisor,
}

impl LightClient {
    pub async fn new() -> LightClient {
        let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
        let node_info = base_client::new(client)
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
            .primary(peer_id, "tcp://127.0.0.1:26657".parse().unwrap(), instance)
            .witness(peer_id, "tcp://127.0.0.1:26657".parse().unwrap(), instance2)
            .inner();

        let supervisor = tendermint_light_client::supervisor::Supervisor::new(
            instances,
            ProdForkDetector::default(),
            EvidenceReporter,
        );
        LightClient { supervisor }
    }

    pub async fn run(&mut self) {
        loop {
            match self.supervisor.verify_to_highest().await {
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
        Box::new(MemoryStore::new()),
        Box::new(LightClientIO::new(peer_id)),
        Box::new(ProdHasher),
        Box::new(WasmClock),
        Box::new(ProdVerifier::default()),
        Box::new(scheduler::basic_bisecting_schedule),
        Box::new(ProdPredicates),
    );

    let builder = builder
        .trust_primary_at(
            config::TRUSTED_HEIGHT.parse().unwrap(),
            config::TRUSTED_HASH.parse().unwrap(),
        )
        .await
        .unwrap();

    let instance = builder.build();

    instance
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
                util::TENDERMINT_HOST,
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

#[async_trait(? Send)]
impl io::Io for LightClientIO {
    async fn fetch_light_block(&self, height: io::AtHeight) -> Result<LightBlock, IoError> {
        let height = match height {
            io::AtHeight::At(height) => height.value(),
            io::AtHeight::Highest => 0,
        };

        let mut param: String = String::new();
        if height != 0 {
            param = format!("?height={}", height)
        }

        let signed_header = reqwest::get(
            format!("http://localhost:26657/commit{}", param).as_str(),
        )
        .await
        .unwrap()
        .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::commit::Response>>()
        .await
        .unwrap()
        .into_result()
        .unwrap()
        .signed_header;

        let height = signed_header.header.height.value();

        let validator_infos = reqwest::get(
            format!("http://localhost:26657/validators?height={}", height).as_str(),
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
            format!("http://localhost:26657/validators?height={}", height + 1).as_str(),
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
