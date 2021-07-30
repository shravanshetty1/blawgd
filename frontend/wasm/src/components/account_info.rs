use crate::blawgd_client;

pub struct AccountInfoComp {
    account_info: blawgd_client::AccountInfo,
}

impl AccountInfoComp {
    pub fn new(account_info: blawgd_client::AccountInfo) -> Box<AccountInfoComp> {
        Box::new(AccountInfoComp { account_info })
    }
}

impl super::Component for AccountInfoComp {
    fn to_html(&self) -> String {
        format!(
            r#"
            <div class="account-info">
                <img id="account-info-photo" src="{}" class="account-info-photo">
                <div id="account-info-name" class="account-info-name">{}</div>
                <div class="account-info-address">@{}</div>
            </div>
"#,
            self.account_info.photo, self.account_info.name, self.account_info.address
        )
    }
}
