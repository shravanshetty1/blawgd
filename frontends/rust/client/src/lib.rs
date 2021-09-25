// uncomment to force warnings
// #![deny(warnings)]
// #![allow(dead_code)]

use wasm_bindgen::{prelude::*, JsValue};
mod components;
use std::sync::Arc;
mod host;
use crate::clients::cosmos_client::CosmosClient;
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

pub mod clients;
mod context;
mod dom;
mod logger;
mod pages;
mod storage;
mod task;

const SYNC_TIMEOUT: u32 = 5000;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    task::spawn_local(main_handler());
    Ok(())
}

pub async fn main_handler() -> Result<()> {
    let window = web_sys::window().ok_or(anyhow!("could not get window object"))?;

    let window = dom::new_window(window);
    let location = window.location();
    let host = Host::new(location.protocol()?, location.hostname()?, location.port()?);
    let grpc_client = grpc_web_client::Client::new(host.grpc_endpoint());
    let peer_id = Store.get_peer_id(grpc_client.clone()).await?;
    let light_client = LightClient::new(peer_id, host.clone()).await?;

    let since = chrono::Utc::now().timestamp_millis() - Store.last_lc_sync()?;
    if since > SYNC_TIMEOUT as i64 + 1000 {
        light_client
            .supervisor
            .write()
            .await
            .verify_to_highest()
            .await?;
    }

    let vc = VerificationClient::new(
        light_client.clone(),
        grpc_client.clone(),
        Store.should_verify()?,
    );
    let session = Store.get_session_account_info(vc.clone()).await.ok();

    let rpc_client = TendermintRPCClient::new(host.clone())?;
    let ctx = Arc::new(ApplicationContext {
        client: MasterClient {
            lc: light_client.clone(),
            vc,
            rpc: rpc_client,
            cosmos: CosmosClient {
                client: grpc_client,
            },
        },
        host,
        store: Store,
        window,
        session,
        logger: Logger,
    });
    PageRenderer::new(ctx)
        .render(location.href()?.as_str())
        .await?;

    light_client.sync_forever(SYNC_TIMEOUT, Store).await?;
    Ok(())
}
