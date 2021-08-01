use crate::blawgd_client;

pub struct AccountInfoComp {
    account_info_view: blawgd_client::AccountInfoView,
}

impl AccountInfoComp {
    pub fn new(account_info: blawgd_client::AccountInfoView) -> Box<AccountInfoComp> {
        Box::new(AccountInfoComp {
            account_info_view: account_info,
        })
    }
}

impl super::Component for AccountInfoComp {
    fn to_html(&self) -> String {
        let account_info = self.account_info_view.account_info.as_ref().unwrap();
        format!(
            r#"
            <div class="account-info">
                <img id="account-info-photo" src="{}" class="account-info-photo">
                <div id="account-info-name" class="account-info-name">{}</div>
                <div class="account-info-address">@{}</div>
                <div class="account-info-follower-info">
                    <a href="/followings/{}" class="account-info-followings">{} Following</a>
                </div>
            </div>
"#,
            account_info.photo,
            account_info.name,
            account_info.address,
            account_info.address,
            self.account_info_view.following_count
        )
    }
}
