use super::Component;
use crate::blawgd_client::AccountInfo;

pub struct FollowingsPage {
    nav_bar: Box<dyn Component>,
    account_infos: Vec<AccountInfo>,
}

impl FollowingsPage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_infos: Vec<AccountInfo>,
    ) -> Box<FollowingsPage> {
        Box::new(FollowingsPage {
            nav_bar,
            account_infos,
        })
    }
}

impl Component for FollowingsPage {
    fn to_html(&self) -> String {
        let mut account_infos_component = String::new();
        for account_info in &self.account_infos {
            account_infos_component = format!(
                r#"
                {}
           <div class="account-info-element">
            <div class="post-component-text-wrapper">
                <a href="/profile/{}"><img src="{}" class="post-component-account-info-image"></a>
                <div class="post-component-text-content">
                    <div class="post-component-account-info">
                        <a href="/profile/{}" class="post-component-account-info-name">{}</a>
                        <div class="post-component-account-info-address">@{}</div>
                    </div>
                </div>
            </div>
            </div> 
            "#,
                account_infos_component,
                account_info.address,
                account_info.photo,
                account_info.address,
                account_info.name,
                account_info.address
            )
        }

        String::from(format!(
            r#"
<div class="page">
    {}
    <div class="main-column">
        <div class="login-page-header with-padding with-border-bottom with-margin-bottom">Followings</div>
        {}
    </div>
    <div class="secondary-column"></div>
</div>
"#,
            self.nav_bar.to_html(),
            account_infos_component,
        ))
    }
}
