use crate::blawgd_client::verification_client::VerificationClient;
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
mod timeline_page;
mod util;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async move {
        let mut lc = light_client::LightClient::new().await;
        lc.supervisor().verify_to_highest().await;
        let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
        let mut cl = VerificationClient::new(&mut lc, client);

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
            url if str::starts_with(url, "profile") => profile_page::handle(&mut cl).await,
            "login" => login_page::handle().await,
            _ => home_page::handle().await,
        };

        lc.run().await
    });

    Ok(())
}
