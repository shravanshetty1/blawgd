use crate::blawgd_client::{AccountInfo, GetAccountInfoRequest};
use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::login_page::LoginPage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::util;
use bip39::{Language, Mnemonic, MnemonicType};
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let storage = window.local_storage().unwrap().unwrap();

    let account_info = util::get_account_info_from_storage(&storage);
    let mut account_info_comp: Option<Box<dyn Component>> = None;
    if account_info.is_some() {
        account_info_comp = Some(AccountInfoComp::new(account_info.clone().unwrap()))
    }

    let nav_bar = NavBar::new(account_info.clone());
    let comp = BlawgdHTMLDoc::new(LoginPage::new(nav_bar, account_info_comp));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    register_event_listeners(&document, &account_info);
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

            storage.remove_item("wallet_mnemonic");
            storage.remove_item("account_info");
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
            storage.set_item("wallet_mnemonic", &mnemonic);

            let address = crw_wallet::crypto::MnemonicWallet::new(&mnemonic, util::COSMOS_DP)
                .unwrap()
                .get_bech32_address("cosmos")
                .unwrap();
            let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
            let account_info = util::get_account_info(client, address.clone()).await;
            let mut encoded_account_info: Vec<u8> = Vec::new();
            prost::Message::encode(&account_info.clone(), &mut encoded_account_info);
            let account_info_as_string = String::from_utf8(encoded_account_info).unwrap();
            storage.set_item("account_info", &account_info_as_string);

            window.location().reload();
        });
    })
    .forget();
}
