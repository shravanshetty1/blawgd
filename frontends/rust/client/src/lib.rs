use tendermint_light_client::{supervisor::Handle, types::PeerId};
use wasm_bindgen::{prelude::*, JsValue};

mod blawgd_client;
mod components;
mod state;
mod util;
use std::sync::Arc;
mod host;

use crate::clients::light_client;
use crate::clients::light_client::LightClient;
use crate::pages::PageRenderer;
use crate::storage::Store;
use crate::{clients::verification_client::VerificationClient, state::State};
use anyhow::{anyhow, Result};
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient as base_client;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;
use host::Host;

mod clients;
mod dom;
mod pages;
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
    let location = window.location();
    let host = Host::new(location.protocol()?, location.hostname()?, location.port()?);
    let grpc_client = grpc_web_client::Client::new(host.grpc_endpoint().into());
    let light_client = LightClient::new(grpc_client.clone(), host.clone()).await?;
    light_client.write().await.verify_to_highest().await?;

    let verification_client = VerificationClient::new(light_client.clone(), grpc_client.clone());
    let page_renderer = PageRenderer::new(host, Store, window, verification_client, grpc_client);
    page_renderer.render(location.href()?.as_str()).await?;

    LightClient::sync_forever(light_client).await;
    Ok(())
}
