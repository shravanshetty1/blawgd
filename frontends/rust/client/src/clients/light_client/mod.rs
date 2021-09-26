use crate::clients::light_client::clock::WasmClock;
use crate::clients::light_client::custom_evidence_reporter::CustomEvidenceReporter;
use crate::clients::light_client::light_client_io::LightClientIO;
use crate::clients::light_client::light_store::CustomLightStore;
use crate::clients::rpc_client::TendermintRPCClient;
use crate::host::Host;
use crate::storage::Store;
use anyhow::anyhow;
use anyhow::Result;
use async_lock::RwLock;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;
use std::sync::Arc;
use std::time::Duration;
use tendermint_light_client::builder::LightClientBuilder;
use tendermint_light_client::builder::SupervisorBuilder;
use tendermint_light_client::components::scheduler;
use tendermint_light_client::components::verifier::ProdVerifier;
use tendermint_light_client::fork_detector::ProdForkDetector;
use tendermint_light_client::operations::ProdHasher;
use tendermint_light_client::predicates::ProdPredicates;
use tendermint_light_client::store::LightStore;
use tendermint_light_client::supervisor::{Instance, Supervisor};
use tendermint_light_client::types::{PeerId, TrustThreshold};
use tendermint_light_client::{light_client, supervisor};
use tendermint_rpc::Url;

mod clock;
mod custom_evidence_reporter;
mod light_client_io;
pub mod light_store;

const TRUSTING_PERIOD: u64 = 3600000;
const CLOCK_DRIFT: u64 = 1;

#[derive(Clone)]
pub struct LightClient {
    pub supervisor: Arc<RwLock<Supervisor>>,
}

impl LightClient {
    pub async fn new(peer_id: PeerId, host: Host) -> Result<LightClient> {
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
        Ok(LightClient {
            supervisor: Arc::new(RwLock::new(supervisor)),
        })
    }
    pub async fn sync_forever(&self, timeout_ms: u32, store: Store) -> Result<()> {
        loop {
            match self.supervisor.write().await.verify_to_highest().await {
                Ok(light_block) => {
                    crate::logger::console_log(
                        format!("[info] synced to block {}", light_block.height()).as_str(),
                    );
                }
                Err(err) => {
                    crate::logger::console_log(format!("[error] sync failed: {}", err).as_str());
                }
            }
            store.update_last_sync_time()?;
            store.prune_light_store()?;
            gloo::timers::future::TimeoutFuture::new(timeout_ms).await;
        }

        Ok(())
    }
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

    let builder = LightClientBuilder::custom(
        peer_id,
        options,
        Box::new(CustomLightStore),
        Box::new(LightClientIO::new(peer_id, rpc_client.clone())),
        Box::new(ProdHasher),
        Box::new(WasmClock),
        Box::new(ProdVerifier::default()),
        Box::new(scheduler::basic_bisecting_schedule),
        Box::new(ProdPredicates),
    );

    let mut instance: Instance;
    if CustomLightStore.highest_trusted_or_verified().is_some() {
        instance = builder.trust_from_store()?.build();
    } else {
        // TODO remove this once trusted height is hard coded
        let block_resp = rpc_client.get_block(0).await?;
        let trusted_height = block_resp.block.header.height.to_string();
        let trusted_hash = block_resp.block_id.hash.to_string();

        instance = builder
            .trust_primary_at(
                trusted_height.as_str().parse()?,
                trusted_hash.as_str().parse()?,
            )
            .await?
            .build();
    }

    Ok(instance)
}
