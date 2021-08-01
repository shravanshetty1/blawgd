use super::Component;

pub struct ProfilePage {
    nav_bar: Box<dyn Component>,
    account_info: Box<dyn Component>,
    show_edit_button: bool,
    posts: Box<[Box<dyn Component>]>,
    following: bool,
    show_button: bool,
}

impl ProfilePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_info: Box<dyn Component>,
        show_edit_button: bool,
        posts: Box<[Box<dyn Component>]>,
        following: bool,
        show_button: bool,
    ) -> Box<ProfilePage> {
        Box::new(ProfilePage {
            nav_bar,
            account_info,
            show_edit_button,
            posts,
            following,
            show_button,
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
        if self.show_button {
            if self.show_edit_button {
                button = r#"<a href="/edit-profile" class="button">Edit Profile</a>"#.into()
            } else {
                if self.following {
                    button = r#"<a id="follow-toggle" class="button">Unfollow</a>"#.into();
                } else {
                    button = r#"<a id="follow-toggle" class="button">Follow</a>"#.into();
                }
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
