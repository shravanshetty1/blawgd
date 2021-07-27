use super::Component;

pub struct LoginPage {
    nav_bar: Box<dyn Component>,
}

impl LoginPage {
    pub fn new(nav_bar: Box<dyn Component>) -> Box<LoginPage> {
        Box::new(LoginPage { nav_bar })
    }
}

impl Component for LoginPage {
    fn to_html(&self) -> String {
        String::from(format!(
            r#"
<div class="page">
    {}
    <div class="main-column">
        <div class="login-page-header">Currently logged in as</div>
        <div class="account-info">
            <img src="profile.jpeg" class="account-info-photo">
            <div class="account-info-name">John Doe</div>
            <div class="account-info-address">@cosm1234312341234</div>
            <div class="button">Logout</div>
        </div>
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
        ))
    }
}
