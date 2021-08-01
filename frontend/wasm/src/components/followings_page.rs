use super::Component;
use crate::blawgd_client::AccountInfoView;

pub struct FollowingsPage {
    nav_bar: Box<dyn Component>,
    account_infos: Vec<AccountInfoView>,
}

impl FollowingsPage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_infos: Vec<AccountInfoView>,
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
        for account_info_view in &self.account_infos {
            let account_info = account_info_view.account_info.as_ref().unwrap();
            account_infos_component = format!(
                r#"
                {}
           <div class="account-info-element">
            <div class="post-component-text-wrapper">
                <img src="{}" class="post-component-account-info-image">
                <div class="post-component-text-content">
                    <div class="post-component-account-info">
                        <div class="post-component-account-info-name">{}</div>
                        <div class="post-component-account-info-address">@{}</div>
                    </div>
                </div>
            </div>
            </div> 
            "#,
                account_infos_component,
                account_info.photo,
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
