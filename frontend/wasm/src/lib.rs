use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;
use std::time::Duration;
use tendermint_light_client::fork_detector::ProdForkDetector;
use tendermint_light_client::supervisor::Handle;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys;

mod blawgd_client;
mod components;
mod edit_profile_page;
mod followings_page;
mod home_page;
mod light_client;
mod login_page;
mod post_page;
mod profile_page;
mod timeline_page;
mod util;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient as base_client;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async move {
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

        let instance = light_client::make_instance(peer_id).await;
        let instance2 = light_client::make_instance(peer_id).await;
        let (instances, addresses) = tendermint_light_client::builder::SupervisorBuilder::new()
            .primary(peer_id, "tcp://127.0.0.1:26657".parse().unwrap(), instance)
            .witness(peer_id, "tcp://127.0.0.1:26657".parse().unwrap(), instance2)
            .inner();

        let mut supervisor = tendermint_light_client::supervisor::Supervisor::new(
            instances,
            ProdForkDetector::default(),
            light_client::EvidenceReporter::new(),
        );

        loop {
            match supervisor.verify_to_highest().await {
                Ok(light_block) => {
                    util::console_log(
                        format!("[info] synced to block {}", light_block.height()).as_str(),
                    );
                }
                Err(err) => {
                    util::console_log(format!("[error] sync failed: {}", err).as_str());
                }
            }

            // std::thread::sleep(Duration::from_millis(800));
        }

        let url: String = web_sys::window().unwrap().location().href().unwrap();
        let url_path = url
            .as_str()
            .strip_prefix(format!("{}/", util::HOST_NAME).as_str())
            .unwrap();

        match url_path {
            url if str::starts_with(url, "followings") => followings_page::handle().await,
            url if str::starts_with(url, "post") => post_page::handle().await,
            "edit-profile" => edit_profile_page::handle().await,
            "timeline" => timeline_page::handle().await,
            url if str::starts_with(url, "profile") => profile_page::handle().await,
            "login" => login_page::handle().await,
            _ => home_page::handle().await,
        };
    });

    Ok(())
}
