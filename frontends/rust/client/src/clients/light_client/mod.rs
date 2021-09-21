use std::time::Duration;

use async_trait::async_trait;
use contracts::contract_trait;
use cosmos_sdk_proto::{
    cosmos::base::tendermint::v1beta1::service_client::ServiceClient as base_client,
    cosmos::base::tendermint::v1beta1::GetNodeInfoRequest,
};

use tendermint::{abci::transaction::Hash, block::Height, evidence::Evidence, net::Address, Time};

use self::custom_evidence_reporter::CustomEvidenceReporter;
use super::rpc_client::TendermintRPCClient;
use crate::{
    clients::light_client::clock::WasmClock, clients::light_client::light_store::CustomLightStore,
    host::Host, util,
};
use anyhow::Result;
use evidence::EvidenceReporter;
use gloo::{storage::errors::StorageError, storage::LocalStorage, storage::Storage};
use light_client_io::LightClientIO;
use supervisor::SupervisorHandle;
use tendermint_light_client::components::clock::Clock;
use tendermint_light_client::{
    builder::{LightClientBuilder, SupervisorBuilder},
    components::{io, io::IoError, scheduler, verifier::ProdVerifier},
    evidence,
    fork_detector::ProdForkDetector,
    light_client,
    operations::ProdHasher,
    predicates::ProdPredicates,
    store::memory::MemoryStore,
    supervisor,
    supervisor::Handle,
    supervisor::Supervisor,
    types::{LightBlock, PeerId, Status, TrustThreshold},
};
use tendermint_rpc::{
    endpoint::{block, commit},
    response::Wrapper,
    Url,
};

mod clock;
mod custom_evidence_reporter;
mod light_client_io;
mod light_store;

const TRUSTING_PERIOD: u64 = 3600000;
const CLOCK_DRIFT: u64 = 1;

pub async fn new(peer_id: PeerId, host: Host) -> Result<Supervisor> {
    let rpc_client = TendermintRPCClient::new(host.clone())?;

    let instance = new_light_client_instance(peer_id, rpc_client.clone()).await?;
    let instance2 = new_light_client_instance(peer_id, rpc_client.clone()).await?;

    let address = host.tendermint_endpoint().parse::<Url>()?;
    let (instances, _) = SupervisorBuilder::new()
        .primary(peer_id, address.clone(), instance)
        .witness(peer_id, address, instance2)
        .inner();

    let supervisor = Supervisor::new(
        instances,
        ProdForkDetector::default(),
        CustomEvidenceReporter::new(rpc_client.clone()),
    );
    Ok(supervisor)
}

async fn new_light_client_instance(
    peer_id: PeerId,
    rpc_client: TendermintRPCClient,
) -> Result<supervisor::Instance> {
    let options = light_client::Options {
        trust_threshold: TrustThreshold::default(),
        trusting_period: Duration::from_secs(TRUSTING_PERIOD),
        clock_drift: Duration::from_secs(CLOCK_DRIFT),
    };

    let block_resp = rpc_client.get_block(0).await?;
    let trusted_height = block_resp.block.header.height.to_string();
    let trusted_hash = block_resp.block_id.hash.to_string();

    let instance = LightClientBuilder::custom(
        peer_id,
        options,
        Box::new(CustomLightStore),
        Box::new(LightClientIO::new(peer_id, rpc_client)),
        Box::new(ProdHasher),
        Box::new(WasmClock),
        Box::new(ProdVerifier::default()),
        Box::new(scheduler::basic_bisecting_schedule),
        Box::new(ProdPredicates),
    )
    .trust_primary_at(
        trusted_height.as_str().parse()?,
        trusted_hash.as_str().parse()?,
    )
    .await?
    .build();

    Ok(instance)
}

pub async fn start_sync(handler: SupervisorHandle) {
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
