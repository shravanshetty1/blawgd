use super::Component;
use crate::blawgd_client::AccountInfo;

pub struct EditProfilePage {
    nav_bar: Box<dyn Component>,
    account_info: Box<dyn Component>,
}

impl EditProfilePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_info: Box<dyn Component>,
    ) -> Box<EditProfilePage> {
        Box::new(EditProfilePage {
            nav_bar,
            account_info,
        })
    }
}

impl Component for EditProfilePage {
    fn to_html(&self) -> String {
        String::from(format!(
            r#"
<div class="page">
    {}
    <div class="main-column">
        <div class="account-info-wrapper">
            {}
            <div class="new-account-info">
                <input id="image-field" class="account-info-field" type="text" placeholder="Image here...">
                <input id="name-field" class="account-info-field" type="text" placeholder="Name here...">
                <div class="new-account-info-buttons">
                    <div id="preview-button" class="button">Preview</div>
                    <div id="reset-button" class="button">Reset</div>
                </div>
            </div>
            <div id="update-profile" class="button">Update Profile</div>
        </div>
    </div>
    <div class="secondary-column"></div>
</div>
"#,
            self.nav_bar.to_html(),
            self.account_info.to_html(),
        ))
    }
}
