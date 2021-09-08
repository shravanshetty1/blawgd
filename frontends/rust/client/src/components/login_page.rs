use super::Component;

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
    fn to_html(&self) -> String {
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
                self.account_info.as_ref().unwrap().to_html()
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
