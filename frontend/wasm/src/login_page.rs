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

    let nav_bar = NavBar::new();
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

    let password_field = document
        .get_element_by_id("wallet-password")
        .expect("password element not found");

    events::EventListener::new(&login_element, "click", move |_| {
        let mnemonic = mnemonic_field.text_content().unwrap();
        let password = password_field.text_content().unwrap();
        let cosmos_dp = "m/44'/118'/0'/0/0";

        storage.set_item("wallet_mnemonic", &mnemonic);
    })
    .forget();
}
