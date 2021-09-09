use crate::components::Component;

pub struct PostPage {
    nav_bar: Box<dyn Component>,
    main_post: Box<dyn Component>,
    post_creator: Option<Box<dyn Component>>,
    posts: Box<[Box<dyn Component>]>,
}

impl PostPage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        main_post: Box<dyn Component>,
        post_creator: Option<Box<dyn Component>>,
        posts: Box<[Box<dyn Component>]>,
    ) -> Box<PostPage> {
        Box::new(PostPage {
            nav_bar,
            main_post,
            post_creator,
            posts,
        })
    }
}

impl super::Component for PostPage {
    fn to_html(&self) -> String {
        let mut posts: String = String::new();
        for post in self.posts.iter() {
            posts = format!("{}{}", posts, post.to_html())
        }

        let mut post_creator: String = String::new();
        if self.post_creator.is_some() {
            post_creator = self.post_creator.as_ref().unwrap().to_html();
        }

        String::from(format!(
            r#"
<div class="page">
    {}
    <div id="main-column" class="main-column">
        {}
        {}
        {}
    </div>
    <div class="secondary-column"></div>
</div>
"#,
            self.nav_bar.to_html(),
            self.main_post.to_html(),
            post_creator,
            posts
        ))
    }
}
