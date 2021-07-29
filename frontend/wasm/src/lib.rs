use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys;
mod blawgd_client;
mod components;
mod home_page;
mod login_page;
mod profile_page;
mod util;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();

    let url: String = window.location().href().unwrap();
    let url_path = url.as_str().strip_prefix("http://localhost:2341/").unwrap();

    match url_path {
        "explore" => profile_page::handle(),
        url if str::starts_with(url, "profile") => profile_page::handle(),
        "login" => login_page::handle(&window),
        _ => home_page::handle(),
    };

    Ok(())
}
