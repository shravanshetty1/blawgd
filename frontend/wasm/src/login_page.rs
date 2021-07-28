use crate::blawgd_client::GetAccountInfoRequest;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::login_page::LoginPage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use bip39::{Language, Mnemonic, MnemonicType};
use gloo::events;

pub fn handle(window: &web_sys::Window) {
    let document = window.document().expect("document missing");
    let storage = window
        .local_storage()
        .expect("storage object missing")
        .unwrap();

    let account_info = super::util::get_account_info_from_storage(&storage);
    let nav_bar = NavBar::new(account_info);
    let comp = BlawgdHTMLDoc::new(LoginPage::new(nav_bar));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    let generate_account = document
        .get_element_by_id("generate-account")
        .expect("generate-account element not found");

    let mnemonic_field = document
        .get_element_by_id("wallet-mnemonic")
        .expect("mnemonic element not found");

    events::EventListener::new(&generate_account, "click", move |_| {
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
        let phrase = mnemonic.phrase().to_owned();

        mnemonic_field.set_text_content(Some(phrase.as_str()));
    })
    .forget();

    let mnemonic_field = document
        .get_element_by_id("wallet-mnemonic")
        .expect("mnemonic element not found");

    let login_element = document
        .get_element_by_id("login")
        .expect("login element not found");

    // let password_field = document
    //     .get_element_by_id("wallet-password")
    //     .expect("password element not found");

    events::EventListener::new(&login_element, "click", move |_| {
        let mnemonic = mnemonic_field.text_content().unwrap();
        // let password = password_field.text_content().unwrap();
        let cosmos_dp = "m/44'/118'/0'/0/0";

        storage.set_item("wallet_mnemonic", &mnemonic);
        wasm_bindgen_futures::spawn_local(async move {
            let wallet = crw_wallet::crypto::MnemonicWallet::new(&mnemonic, cosmos_dp).unwrap();
            let address = wallet.get_bech32_address("cosmos").unwrap();
            let client = grpc_web_client::Client::new("http://localhost:9091".into());

            let resp = super::blawgd_client::query_client::QueryClient::new(client)
                .get_account_info(GetAccountInfoRequest {
                    address: address.clone(),
                })
                .await
                .unwrap();

            let mut account_info = resp.get_ref().account_info.as_ref().unwrap().clone();
            account_info.address = address;

            let mut encoded_account_info: Vec<u8> = Vec::new();
            prost::Message::encode(&account_info, &mut encoded_account_info);

            let account_info_as_string = String::from_utf8(encoded_account_info).unwrap();
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            storage.set_item("account_info", &account_info_as_string);
            window.location().reload();
        });
    })
    .forget();
}
