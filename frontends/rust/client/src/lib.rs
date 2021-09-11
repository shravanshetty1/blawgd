use crate::blawgd_client::verification_client::VerificationClient;
use crate::state::State;
use tendermint_light_client::supervisor::Handle;
use wasm_bindgen::{prelude::*, JsValue};
use web_sys;

mod blawgd_client;
mod components;
mod config;
mod edit_profile_page;
mod followings_page;
mod home_page;
mod light_client;
mod login_page;
mod post_page;
mod profile_page;
mod state;
mod storage;
mod timeline_page;
mod util;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    // TODO move event listeners inside components
    // TODO should not refresh page when visiting another page

    wasm_bindgen_futures::spawn_local(async move {
        let client = grpc_web_client::Client::new(crate::config::GRPC_WEB_ADDRESS.into());

        let mut supervisor = light_client::new_supervisor(client.clone()).await;
        let light_client = supervisor.handle();
        wasm_bindgen_futures::spawn_local(async move {
            supervisor.run().await;
            ()
        });
        light_client.verify_to_highest().await;

        let cl = VerificationClient::new(light_client.clone(), client.clone());

        let url: String = web_sys::window().unwrap().location().href().unwrap();
        let url_path = url
            .as_str()
            .strip_prefix(format!("{}/", crate::config::HOST_NAME).as_str())
            .unwrap();

        let result = match url_path {
            url if str::starts_with(url, "followings") => followings_page::handle(cl).await,
            url if str::starts_with(url, "post") => post_page::handle(cl).await,
            "edit-profile" => edit_profile_page::handle(cl).await,
            "timeline" => timeline_page::handle(cl).await,
            url if str::starts_with(url, "profile") => profile_page::handle(cl).await,
            "login" => login_page::handle(cl).await,
            _ => home_page::handle(cl).await,
        };
        result.unwrap();

        crate::state::set_state(State { page: 1 });

        light_client::start_sync(light_client).await;
        ()
    });

    Ok(())
}
