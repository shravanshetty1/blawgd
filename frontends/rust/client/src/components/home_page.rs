use crate::components::Component;
use crate::context::ApplicationContext;
use anyhow::Result;
use std::sync::Arc;

pub struct HomePage {
    nav_bar: Box<dyn Component>,
    post_creator: Option<Box<dyn Component>>,
    posts: Box<[Box<dyn Component>]>,
}

impl HomePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        post_creator: Option<Box<dyn Component>>,
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
    </div>
    <div class="secondary-column"></div>
</div>
"#,
            self.nav_bar.to_html(),
            post_creator,
            posts
        ))
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.nav_bar.register_events(ctx.clone())?;
        if self.post_creator.is_some() {
            self.post_creator
                .as_ref()
                .unwrap()
                .register_events(ctx.clone())?;
        }
        for p in self.posts.iter() {
            p.register_events(ctx.clone())?;
        }

        Ok(())
    }
}
