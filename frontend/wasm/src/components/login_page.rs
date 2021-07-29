use super::Component;
use crate::blawgd_client::AccountInfo;

pub struct LoginPage {
    nav_bar: Box<dyn Component>,
    account_info: Option<AccountInfo>,
}

impl LoginPage {
    pub fn new(nav_bar: Box<dyn Component>, account_info: Option<AccountInfo>) -> Box<LoginPage> {
        Box::new(LoginPage {
            nav_bar,
            account_info,
        })
    }
}

impl Component for LoginPage {
    fn to_html(&self) -> String {
        let mut account_info_component = String::new();
        if self.account_info.is_some() {
            let mut account_info = self.account_info.as_ref().unwrap().clone();

            if account_info.photo.is_empty() {
                account_info.photo = "profile.jpeg".into()
            }

            account_info_component = String::from(format!(
                r#"
                <div class="login-page-header">Currently logged in as</div>
                <div class="account-info">
                    <img src="{}" class="account-info-photo">
                    <div class="account-info-name">{}</div>
                    <div class="account-info-address">@{}</div>
                    <div class="button">Logout</div>
                </div>
                "#,
                account_info.photo, account_info.name, account_info.address
            ))
        }

        String::from(format!(
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
            self.nav_bar.to_html(),
            account_info_component
        ))
    }
}
