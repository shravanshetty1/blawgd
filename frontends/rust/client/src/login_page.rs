use crate::{
    components::account_info::AccountInfoComp, components::blawgd_html::BlawgdHTMLDoc,
    components::login_page::LoginPage, components::nav_bar::NavBar, components::Component, util,
    util::StoredData,
};
use bip39::{Language, Mnemonic, MnemonicType};

use crate::blawgd_client::verification_client::VerificationClient;
use crate::blawgd_client::AccountInfo;
use anyhow::Result;
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle(cl: VerificationClient) -> Result<()> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let storage = window.local_storage().unwrap().unwrap();

    let account_info = util::get_session_account_info(&storage, cl).await;
    let mut account_info_comp: Option<Box<dyn Component>> = None;
    if account_info.is_some() {
        account_info_comp = Some(AccountInfoComp::new(account_info.clone().unwrap()))
    }

    let nav_bar = NavBar::new(account_info.clone());
    let comp = BlawgdHTMLDoc::new(LoginPage::new(nav_bar, account_info_comp));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    register_event_listeners(&document, &account_info);
    Ok(())
}

fn register_event_listeners(document: &web_sys::Document, account_info: &Option<AccountInfo>) {
    let generate_account = document
        .get_element_by_id("generate-account")
        .expect("generate-account element not found");

    events::EventListener::new(&generate_account, "click", move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let phrase = Mnemonic::new(MnemonicType::Words24, Language::English)
            .phrase()
            .to_owned();

        let mnemonic_field = document
            .get_element_by_id("wallet-mnemonic")
            .expect("mnemonic element not found");
        mnemonic_field.set_text_content(Some(phrase.as_str()));
    })
    .forget();

    if account_info.is_some() {
        let logout_button = document
            .get_element_by_id("logout-button")
            .expect("logout element not found");

        events::EventListener::new(&logout_button, "click", move |_| {
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();

            util::remove_stored_data(&storage);
            window.location().reload();
        })
        .forget();
    }

    let login_element = document
        .get_element_by_id("login")
        .expect("login element not found");

    events::EventListener::new(&login_element, "click", move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let storage = window.local_storage().unwrap().unwrap();

            let mnemonic_field = document
                .get_element_by_id("wallet-mnemonic")
                .expect("mnemonic element not found");
            let mnemonic: String = mnemonic_field
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .value();

            let address = crw_wallet::crypto::MnemonicWallet::new(&mnemonic, util::COSMOS_DP)
                .unwrap()
                .get_bech32_address("cosmos")
                .unwrap();

            let resp = reqwest::get(&format!(
                "{}/?address={}",
                crate::config::FAUCET_ADDR,
                address
            ))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
            util::console_log(resp.as_str());

            util::set_stored_data(&storage, StoredData { mnemonic, address });
            window.location().reload();
        });
    })
    .forget();
}
