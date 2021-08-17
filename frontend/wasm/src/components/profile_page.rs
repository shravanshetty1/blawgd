use super::Component;

pub struct ProfilePage {
    nav_bar: Box<dyn Component>,
    account_info: Box<dyn Component>,
    button: Option<ButtonType>,
    posts: Box<[Box<dyn Component>]>,
}

#[derive(Clone)]
pub enum ButtonType {
    Edit,
    Follow,
    Unfollow,
}

impl ProfilePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_info: Box<dyn Component>,
        button: Option<ButtonType>,
        posts: Box<[Box<dyn Component>]>,
    ) -> Box<ProfilePage> {
        Box::new(ProfilePage {
            nav_bar,
            account_info,
            button,
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

        let mut button = String::new();
        if self.button.is_some() {
            button = match self.button.as_ref().unwrap() {
                ButtonType::Edit => {
                    r#"<a href="/edit-profile" class="button">Edit Profile</a>"#.into()
                }

                ButtonType::Unfollow => {
                    r#"<a id="follow-toggle" class="button">Unfollow</a>"#.into()
                }
                ButtonType::Follow => r#"<a id="follow-toggle" class="button">Follow</a>"#.into(),
            }
        }

        let account_info_component = String::from(format!(
            r#"
                <div class="account-info-wrapper">
                    {}
                    {}
                </div>
                "#,
            self.account_info.to_html(),
            button
        ));

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
