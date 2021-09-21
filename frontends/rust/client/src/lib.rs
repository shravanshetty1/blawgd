use tendermint_light_client::{supervisor::Handle, types::PeerId};
use wasm_bindgen::{prelude::*, JsValue};

mod blawgd_client;
mod components;
mod edit_profile_page;
mod followings_page;
mod home_page;
mod login_page;
mod post_page;
mod profile_page;
mod state;
mod timeline_page;
mod util;
use std::sync::Arc;
mod host;

use crate::clients::light_client;
use crate::storage::Store;
use crate::{clients::verification_client::VerificationClient, state::State};
use anyhow::{anyhow, Result};
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient as base_client;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;
use host::Host;

mod clients;
mod dom;
mod storage;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    // TODO move event listeners inside components
    // TODO should not refresh page when visiting another page

    wasm_bindgen_futures::spawn_local(async move {
        // TODO add error handling
        main_handler().await.unwrap();
    });

    Ok(())
}

pub async fn main_handler() -> Result<()> {
    let window = web_sys::window().ok_or(anyhow!("could not get window object"))?;
    let window = dom::new_window(window);
    let host = Host::new(
        window.location().protocol()?,
        window.location().hostname()?,
        window.location().port()?,
    );
    let grpc_client = grpc_web_client::Client::new(host.grpc_endpoint().into());

    let node_info = base_client::new(grpc_client.clone())
        .get_node_info(GetNodeInfoRequest {})
        .await?
        .get_ref()
        .clone()
        .default_node_info
        .ok_or(anyhow!("could not get node info"))?;
    let peer_id = node_info.default_node_id.parse::<PeerId>()?;
    let lc = light_client::new(peer_id, host.clone()).await?;
    lc.write().await.verify_to_highest().await?;
    let cl = VerificationClient::new(lc.clone(), grpc_client.clone());

    let url = window.location().href()?;
    let url_path = url
        .as_str()
        .strip_prefix(format!("{}/", host.endpoint()).as_str())
        .ok_or(anyhow!("could not stip prefix of {}", url))?;
    match url_path {
        url if str::starts_with(url, "followings") => {
            followings_page::handle(Store, host, cl).await
        }
        url if str::starts_with(url, "post") => post_page::handle(Store, host, cl).await,
        url if str::starts_with(url, "edit-profile") => {
            edit_profile_page::handle(Store, host, cl).await
        }
        url if str::starts_with(url, "timeline") => timeline_page::handle(host, Store, cl).await,
        url if str::starts_with(url, "profile") => profile_page::handle(Store, host, cl).await,
        url if str::starts_with(url, "login") => login_page::handle(Store, host, cl).await,
        _ => home_page::handle(host, Store, window, cl).await,
    }?;

    light_client::start_sync(lc).await;
    Ok(())
}
