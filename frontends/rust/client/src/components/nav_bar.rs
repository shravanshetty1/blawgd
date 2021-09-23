use crate::clients::blawgd_client::AccountInfo;
use crate::context::ApplicationContext;
use anyhow::Result;
use std::sync::Arc;

pub struct NavBar {
    account_info: Option<AccountInfo>,
}

impl NavBar {
    pub fn new(account_info: Option<AccountInfo>) -> Box<NavBar> {
        Box::new(NavBar { account_info })
    }
}

impl super::Component for NavBar {
    fn to_html(&self) -> String {
        let mut account_menu_items: String = String::new();
        let mut login_component: String = String::from(
            r#"
            <a href="/login" class="login-link-component-wrapper">
                <img src="/profile.jpeg" class="post-component-account-info-image">
                <div class="login-link-component-text">Login/Logout</div>
            </a>
            "#,
        );

        if self.account_info.is_some() {
            let account_info = self.account_info.as_ref().unwrap().clone();
            account_menu_items = String::from(format!(
                r#"
            <a href="/timeline" class="nav-bar-menu-element">Timeline</a>
            <a href="/profile/{}" class="nav-bar-menu-element">Profile</a> 
            "#,
                account_info.address
            ));

            let mut login_comp_text = account_info.name.clone();

            if login_comp_text.is_empty() {
                login_comp_text = account_info.address.clone();
            }

            login_component = String::from(format!(
                r#"
            <a href="/login" class="login-link-component-wrapper">
                <img src="{}" class="post-component-account-info-image">
                <div class="login-link-component-text">{}</div>
            </a>
            "#,
                account_info.photo.clone(),
                login_comp_text
            ));
        }

        String::from(format!(
            r#"
    <div class="nav-bar">
        <a href="/" class="nav-bar-header">
            Blawgd
        </a>
        <div class="nav-bar-menu">
            <a href="/" class="nav-bar-menu-element">Explore</a>
            {}
            <a href="/about" class="nav-bar-menu-element">About</a>
        </div>
        {}
    </div>"#,
            account_menu_items, login_component
        ))
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        Ok(())
    }
}
