use crate::blawgd_client::GetAccountInfoRequest;
use crate::{
    edit_profile_page, followings_page, home_page, login_page, post_page, profile_page,
    timeline_page, util,
};
use contracts::contract_trait;

use async_trait::async_trait;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::{
    GetNodeInfoRequest, GetValidatorSetByHeightRequest,
};
use reqwest::Url;
use std::future::Future;
use std::time::Duration;
use tendermint::abci::transaction::Hash;
use tendermint::channel::Serialize;
use tendermint::evidence::Evidence;
use tendermint::validator::Info;
use tendermint::Time;
use tendermint_light_client::components::clock::SystemClock;
use tendermint_light_client::components::io;
use tendermint_light_client::components::io::IoError;
use tendermint_light_client::components::scheduler;
use tendermint_light_client::components::verifier::ProdVerifier;
use tendermint_light_client::fork_detector::ProdForkDetector;
use tendermint_light_client::light_client;
use tendermint_light_client::operations::ProdHasher;
use tendermint_light_client::peer_list::PeerListBuilder;
use tendermint_light_client::predicates::ProdPredicates;
use tendermint_light_client::store::memory::MemoryStore;
use tendermint_light_client::supervisor::Handle;
use tendermint_light_client::types::{LightBlock, PeerId, TrustThreshold};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys;

const TRUSTED_HEIGHT: &str = "13284";
const TRUSTED_HASH: &str = "C34D2576BF6CB817706D5C6FED9D9C5BBEEBFF255D33E860EC0A95B3809FD267";

pub(crate) async fn make_instance(
    peer_id: PeerId,
) -> tendermint_light_client::supervisor::Instance {
    let options = light_client::Options {
        trust_threshold: TrustThreshold::default(),
        trusting_period: Duration::from_secs(360000),
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
            TRUSTED_HEIGHT.parse().unwrap(),
            TRUSTED_HASH.parse().unwrap(),
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

const TENDERMINT_HOST: &str = "http://localhost:26657";

pub struct EvidenceReporter {}

impl EvidenceReporter {
    pub(crate) fn new() -> EvidenceReporter {
        EvidenceReporter {}
    }
}

#[contract_trait]
#[async_trait(?Send)]
impl tendermint_light_client::evidence::EvidenceReporter for EvidenceReporter {
    async fn report(&self, e: Evidence, peer: PeerId) -> Result<Hash, IoError> {
        let evidence = serde_json::to_string(&e).unwrap();

        let resp = reqwest::get(
            format!(
                "{}/broadcast_evidence?evidence={}",
                TENDERMINT_HOST, evidence
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

#[async_trait(?Send)]
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
