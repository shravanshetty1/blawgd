use crate::components::Component;

pub struct HomePage {
    nav_bar: Box<dyn Component>,
    post_creator: Box<dyn Component>,
    posts: Box<[Box<dyn Component>]>,
}

impl HomePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        post_creator: Box<dyn Component>,
        posts: Box<[Box<dyn Component>]>,
    ) -> Box<HomePage> {
        Box::new(HomePage {
            nav_bar,
            post_creator,
            posts,
        })
    }
}

impl super::Component for HomePage {
    fn to_html(&self) -> String {
        let mut posts: String = String::new();
        for post in self.posts.iter() {
            posts = format!("{}{}", posts, post.to_html())
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
            self.post_creator.to_html(),
            posts
        ))
    }
}
