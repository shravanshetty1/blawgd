use wasm_bindgen::{prelude::*, JsValue};
mod components;
use std::sync::Arc;
mod host;
use crate::clients::light_client::LightClient;
use crate::clients::rpc_client::TendermintRPCClient;
use crate::clients::verification_client::VerificationClient;
use crate::clients::MasterClient;
use crate::context::ApplicationContext;
use crate::logger::Logger;
use crate::pages::PageRenderer;
use crate::storage::Store;
use anyhow::{anyhow, Result};
use host::Host;

mod clients;
mod context;
mod dom;
mod logger;
mod pages;
mod storage;
mod task;

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
    let grpc_client = grpc_web_client::Client::new(host.grpc_endpoint());
    let light_client = LightClient::new(grpc_client.clone(), host.clone()).await?;
    light_client.write().await.verify_to_highest().await?;

    let rpc_client = TendermintRPCClient::new(host.clone())?;
    let vc = VerificationClient::new(light_client.clone(), grpc_client.clone());
    let ctx = Arc::new(ApplicationContext {
        client: MasterClient {
            lc: light_client.clone(),
            vc: vc.clone(),
            rpc: rpc_client,
            grpc: grpc_client,
        },
        host,
        store: Store,
        window,
        session: Store.get_session_account_info(vc).await.ok(),
        logger: Logger,
    });
    PageRenderer::new(ctx)
        .render(location.href()?.as_str())
        .await?;

    LightClient::sync_forever(light_client).await;
    Ok(())
}
