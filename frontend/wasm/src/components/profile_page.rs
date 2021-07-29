use super::Component;
use crate::blawgd_client::AccountInfo;

pub struct ProfilePage {
    nav_bar: Box<dyn Component>,
    account_info: Option<AccountInfo>,
    show_edit_button: bool,
    posts: Box<[Box<dyn Component>]>,
}

impl ProfilePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_info: Option<AccountInfo>,
        show_edit_button: bool,
        posts: Box<[Box<dyn Component>]>,
    ) -> Box<ProfilePage> {
        Box::new(ProfilePage {
            nav_bar,
            account_info,
            show_edit_button,
            posts,
        })
    }
}

impl Component for ProfilePage {
    fn to_html(&self) -> String {
        let mut posts: String = String::new();
        for post in self.posts.iter() {
            posts = format!("{}{}", posts, post.to_html())
        }

        let mut account_info_component = String::new();
        if self.account_info.is_some() {
            let mut account_info = self.account_info.as_ref().unwrap().clone();

            let mut edit_button = String::new();
            if self.show_edit_button {
                edit_button = r#"<a href="/edit-profile" class="button">Edit Profile</a>"#.into()
            }

            account_info_component = String::from(format!(
                r#"
                <div class="account-info">
                    <img src="{}" class="account-info-photo">
                    <div class="account-info-name">{}</div>
                    <div class="account-info-address">@{}</div>
                    {}
                </div>
                "#,
                account_info.photo, account_info.name, account_info.address, edit_button
            ))
        }

        String::from(format!(
            r#"
<div class="page">
    {}
    <div class="main-column">
        {}
        {}
    </div>
    <div class="secondary-column"></div>
</div>
"#,
            self.nav_bar.to_html(),
            account_info_component,
            posts
        ))
    }
}
