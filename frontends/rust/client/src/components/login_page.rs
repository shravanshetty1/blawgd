use super::Component;
use crate::context::ApplicationContext;
use crate::storage::{ApplicationData, COSMOS_DP};
use crate::task;
use anyhow::anyhow;
use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType};
use crw_wallet::crypto::MnemonicWallet;
use gloo::events;
use prost::alloc::sync::Arc;
use task::spawn_local;
use wasm_bindgen::JsCast;

pub struct LoginPage {
    nav_bar: Box<dyn Component>,
    account_info: Option<Box<dyn Component>>,
}

impl LoginPage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_info: Option<Box<dyn Component>>,
    ) -> Box<LoginPage> {
        Box::new(LoginPage {
            nav_bar,
            account_info,
        })
    }
}

impl Component for LoginPage {
    fn to_html(&self) -> Result<String> {
        let mut account_info_component = String::new();
        if self.account_info.is_some() {
            account_info_component = String::from(format!(
                r#"
                <div class="login-page-header">Currently logged in as</div>
                <div class="account-info-wrapper">
                    {}
                    <div id="logout-button" class="button">Logout</div>
                </div>
                "#,
                self.account_info.as_ref().unwrap().to_html()?
            ))
        }

        let html = String::from(format!(
            r#"
<div class="page">
    {}
    <div class="main-column">
        {}
        <div class="login-component">
            <textarea id="wallet-mnemonic" class="login-component-mnemonic" placeholder="Mnemonic here..."></textarea>
            <input id="wallet-password" class="login-component-password" placeholder="Password here...">
            <div class="login-component-buttons">
                <div id="generate-account" class="button">Generate Account</div>
                <div id="login" class="button">Login</div>
            </div>
        </div>
    </div>
    <div class="secondary-column"></div>
</div>
"#,
            self.nav_bar.to_html()?,
            account_info_component
        ));

        Ok(html)
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        let document = ctx.window.document()?;
        let generate_account = document.get_element_by_id("generate-account")?.inner();
        events::EventListener::new(&generate_account, "click", move |_| {
            let document = document.clone();
            spawn_local(async move {
                let phrase = Mnemonic::new(MnemonicType::Words24, Language::English)
                    .phrase()
                    .to_owned();

                let mnemonic_field = document.get_element_by_id("wallet-mnemonic")?.inner();
                mnemonic_field.set_text_content(Some(phrase.as_str()));
                Ok(())
            });
        })
        .forget();

        let session = ctx.session.clone();
        if session.is_some() {
            let document = ctx.window.document()?;
            let logout_button = document.get_element_by_id("logout-button")?.inner();
            let store = ctx.store.clone();
            let location = ctx.window.location().inner().clone();
            events::EventListener::new(&logout_button, "click", move |_| {
                store.delete_application_data();
                location.reload().unwrap();
            })
            .forget();
        }

        let document = ctx.window.document()?;
        let login_element = document.get_element_by_id("login")?.inner();
        events::EventListener::new(&login_element, "click", move |_| {
            let ctx = ctx.clone();
            let document = document.clone();
            spawn_local(async move {
                let mnemonic: String = document
                    .get_element_by_id("wallet-mnemonic")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                let address = MnemonicWallet::new(mnemonic.as_str(), COSMOS_DP)?
                    .get_bech32_address("cosmos")?;

                let resp = reqwest::get(&format!(
                    "{}/?address={}",
                    ctx.host.faucet_endpoint(),
                    address
                ))
                .await?
                .text()
                .await?;
                crate::logger::console_log(resp.as_str());

                ctx.store.set_application_data(ApplicationData {
                    mnemonic: mnemonic.to_string(),
                    address,
                })?;
                ctx.window
                    .location()
                    .inner()
                    .reload()
                    .map_err(|_| anyhow!("could not reload page"))?;
                Ok(())
            });
        })
        .forget();

        Ok(())
    }
}
