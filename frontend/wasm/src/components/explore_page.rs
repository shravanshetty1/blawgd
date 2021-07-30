use crate::components::Component;

pub struct ExplorePage {
    nav_bar: Box<dyn Component>,
    posts: Box<[Box<dyn Component>]>,
}

impl ExplorePage {
    pub fn new(nav_bar: Box<dyn Component>, posts: Box<[Box<dyn Component>]>) -> Box<ExplorePage> {
        Box::new(ExplorePage { nav_bar, posts })
    }
}

impl super::Component for ExplorePage {
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
    </div>
    <div class="secondary-column"></div>
</div>
"#,
            self.nav_bar.to_html(),
            posts
        ))
    }
}
