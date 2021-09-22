use crate::components::post::PostComponent;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::task;
use anyhow::Result;
use async_lock::RwLock;
use gloo::events;
use std::sync::Arc;
use task::spawn_local;

pub struct HomePage {
    nav_bar: Box<dyn Component>,
    post_creator: Option<Box<dyn Component>>,
    posts: Box<[Box<dyn Component>]>,
    state: Arc<RwLock<PageState>>,
}

struct PageState {
    page: u64,
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
            state: Arc::new(RwLock::new(PageState { page: 1 })),
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

        let window = ctx.window.inner();
        let state = self.state.clone();
        events::EventListener::new(&window, "scroll", move |_| {
            let ctx = ctx.clone();
            let state = state.clone();
            spawn_local(async move {
                let document = ctx.window.document()?;
                let doc = document.inner().document_element().unwrap();
                let scroll_top: i32 = doc.scroll_top();
                let scroll_height: i32 = doc.scroll_height();
                let client_height: i32 = doc.client_height();

                if scroll_top + client_height >= scroll_height {
                    let mut state = state.write().await;
                    let posts = ctx
                        .client
                        .vc
                        .get_post_by_parent_post("".to_string(), state.page.clone() as u64 + 1)
                        .await?;
                    if posts.len() == 0 {
                        return Ok(());
                    }

                    let mut posts_html: String = String::new();
                    for post in posts {
                        posts_html =
                            format!("{}{}", posts_html, PostComponent::new(post).to_html());
                    }

                    let main_column = document.get_element_by_id("main-column")?.inner();
                    main_column.insert_adjacent_html("beforeend", posts_html.as_str());

                    state.page += 1;
                    crate::logger::console_log(format!("{}", state.page).as_str());
                }

                Ok(())
            });
        })
        .forget();

        Ok(())
    }
}
