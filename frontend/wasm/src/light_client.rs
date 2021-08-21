// use crate::blawgd_client::GetAccountInfoRequest;
// use crate::{
//     edit_profile_page, followings_page, home_page, login_page, post_page, profile_page,
//     timeline_page, util,
// };
// use contracts::contract_trait;
// use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient as base_client;
// use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::{
//     GetNodeInfoRequest, GetValidatorSetByHeightRequest,
// };
// use reqwest::Url;
// use std::future::Future;
// use std::time::Duration;
// use tendermint::abci::transaction::Hash;
// use tendermint::channel::Serialize;
// use tendermint::evidence::Evidence;
// use tendermint::validator::Info;
// use tendermint_light_client::components::clock::SystemClock;
// use tendermint_light_client::components::io;
// use tendermint_light_client::components::io::IoError;
// use tendermint_light_client::components::scheduler;
// use tendermint_light_client::components::verifier::ProdVerifier;
// use tendermint_light_client::fork_detector::ProdForkDetector;
// use tendermint_light_client::light_client;
// use tendermint_light_client::operations::ProdHasher;
// use tendermint_light_client::peer_list::PeerListBuilder;
// use tendermint_light_client::predicates::ProdPredicates;
// use tendermint_light_client::store::memory::MemoryStore;
// use tendermint_light_client::supervisor::Handle;
// use tendermint_light_client::types::{LightBlock, PeerId, TrustThreshold};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsValue;
// use web_sys;
//
// const TRUSTED_HEIGHT: &str = "13284";
// const TRUSTED_HASH: &str = "C34D2576BF6CB817706D5C6FED9D9C5BBEEBFF255D33E860EC0A95B3809FD267";
//
// #[wasm_bindgen(start)]
// pub fn main() -> Result<(), JsValue> {
//     console_error_panic_hook::set_once();
//     wasm_bindgen_futures::spawn_local(async move {
//         let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
//         let node_info = base_client::new(client)
//             .get_node_info(GetNodeInfoRequest {})
//             .await
//             .unwrap()
//             .get_ref()
//             .clone()
//             .default_node_info
//             .unwrap();
//         let peer_id = node_info.default_node_id.parse().unwrap();
//
//         let instance = make_instance(peer_id).await;
//         let instance2 = make_instance(peer_id).await;
//         let (instances, addresses) = tendermint_light_client::builder::SupervisorBuilder::new()
//             .primary(peer_id, "tcp://127.0.0.1:26657".parse().unwrap(), instance)
//             .witness(peer_id, "tcp://127.0.0.1:26657".parse().unwrap(), instance2)
//             .inner();
//
//         util::console_log("starting sync");
//         let supervisor = tendermint_light_client::supervisor::Supervisor::new(
//             instances,
//             ProdForkDetector::default(),
//             EvidenceReporter::new(),
//         );
//
//         let handle = supervisor.handle();
//
//         util::console_log("starting sync");
//         wasm_bindgen_futures::spawn_local(async {
//             supervisor.run();
//             ()
//         });
//
//         // wasm_bindgen_futures::spawn_local(async {
//         //     loop {
//         //         let x = handle.latest_status().unwrap();
//         //         util::console_log(format!("{}", serde_json::to_string(&x).unwrap()).as_ref());
//         //         std::thread::sleep(Duration::from_millis(800));
//         //     }
//         // });
//
//         util::console_log("starting sync");
//         loop {
//             match handle.verify_to_highest() {
//                 Ok(light_block) => {
//                     util::console_log(
//                         format!("[info] synced to block {}", light_block.height()).as_str(),
//                     );
//                 }
//                 Err(err) => {
//                     util::console_log(format!("[error] sync failed: {}", err).as_str());
//                 }
//             }
//
//             std::thread::sleep(Duration::from_millis(800));
//         }
//
//         let url: String = web_sys::window().unwrap().location().href().unwrap();
//         let url_path = url
//             .as_str()
//             .strip_prefix(format!("{}/", util::HOST_NAME).as_str())
//             .unwrap();
//
//         match url_path {
//             url if str::starts_with(url, "followings") => followings_page::handle().await,
//             url if str::starts_with(url, "post") => post_page::handle().await,
//             "edit-profile" => edit_profile_page::handle().await,
//             "timeline" => timeline_page::handle().await,
//             url if str::starts_with(url, "profile") => profile_page::handle().await,
//             "login" => login_page::handle().await,
//             _ => home_page::handle().await,
//         };
//     });
//
//     Ok(())
// }
//
// async fn make_instance(peer_id: PeerId) -> tendermint_light_client::supervisor::Instance {
//     util::console_log("starting sync");
//     let options = light_client::Options {
//         trust_threshold: TrustThreshold::default(),
//         trusting_period: Duration::from_secs(36000),
//         clock_drift: Duration::from_secs(1),
//     };
//     let builder = tendermint_light_client::builder::LightClientBuilder::custom(
//         peer_id,
//         options,
//         Box::new(MemoryStore::new()),
//         Box::new(LightClientIO::new(peer_id)),
//         Box::new(ProdHasher),
//         Box::new(SystemClock),
//         Box::new(ProdVerifier::default()),
//         Box::new(scheduler::basic_bisecting_schedule),
//         Box::new(ProdPredicates),
//     );
//
//     let builder = builder
//         .trust_primary_at(
//             TRUSTED_HEIGHT.parse().unwrap(),
//             TRUSTED_HASH.parse().unwrap(),
//         )
//         .unwrap();
//
//     util::console_log("starting sync");
//     let instance = builder.build();
//
//     instance
// }
//
// const TENDERMINT_HOST: &str = "http://localhost:26657";
//
// pub struct EvidenceReporter {}
//
// impl EvidenceReporter {
//     fn new() -> EvidenceReporter {
//         EvidenceReporter {}
//     }
// }
//
// #[contract_trait]
// impl tendermint_light_client::evidence::EvidenceReporter for EvidenceReporter {
//     fn report(&self, e: Evidence, peer: PeerId) -> Result<Hash, IoError> {
//         let evidence = serde_json::to_string(&e).unwrap();
//
//         let resp = wasm_rs_async_executor::single_threaded::block_on(async {
//             reqwest::get(
//                 format!(
//                     "{}/broadcast_evidence?evidence={}",
//                     TENDERMINT_HOST, evidence
//                 )
//                     .as_str(),
//             )
//                 .await
//                 .unwrap()
//                 .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::evidence::Response>>()
//                 .await
//                 .unwrap()
//                 .into_result()
//                 .unwrap()
//         });
//
//         Ok(resp.hash)
//     }
// }
//
// pub struct LightClientIO {
//     peer_id: PeerId,
// }
//
// impl LightClientIO {
//     fn new(id: PeerId) -> LightClientIO {
//         LightClientIO { peer_id: id }
//     }
// }
//
// impl io::Io for LightClientIO {
//     fn fetch_light_block(&self, height: io::AtHeight) -> Result<LightBlock, IoError> {
//         let height = match height {
//             io::AtHeight::At(height) => height.value(),
//             io::AtHeight::Highest => 0,
//         };
//
//         util::console_log("something1");
//         let signed_header = wasm_rs_async_executor::single_threaded::block_on(async move {
//             reqwest::get(format!("http://localhost:26657/commit?height={}", height).as_str()).await
//                 .unwrap()
//                 .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::commit::Response>>()
//                 .await
//                 .unwrap()
//                 .into_result()
//                 .unwrap()
//                 .signed_header
//         });
//         util::console_log("something2");
//
//         let height = signed_header.header.height.value();
//
//         let validator_infos = wasm_rs_async_executor::single_threaded::block_on(async move {
//             reqwest::get(
//                 format!("http://localhost:26657/validators?height={}", height).as_str(),
//             )
//                 .await
//                 .unwrap()
//                 .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::validators::Response>>()
//                 .await
//                 .unwrap()
//                 .into_result()
//                 .unwrap()
//                 .validators
//         });
//
//         let validators = tendermint::validator::Set::with_proposer(
//             validator_infos,
//             signed_header.header.proposer_address,
//         )
//         .unwrap();
//
//         let next_validator_infos = wasm_rs_async_executor::single_threaded::block_on(async move {
//             reqwest::get(
//                 format!("http://localhost:26657/validators?height={}", height + 1).as_str(),
//             )
//                 .await
//                 .unwrap()
//                 .json::<tendermint_rpc::response::Wrapper<tendermint_rpc::endpoint::validators::Response>>()
//                 .await
//                 .unwrap()
//                 .into_result()
//                 .unwrap()
//                 .validators
//         });
//
//         let next_validators = tendermint::validator::Set::without_proposer(next_validator_infos);
//
//         Ok(LightBlock {
//             signed_header,
//             validators,
//             next_validators,
//             provider: self.peer_id,
//         })
//     }
// }
