use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::PostComponent;
use crate::components::post_creator::PostCreator;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageRenderer;

use crate::clients::blawgd_client::PostView;
use anyhow::Result;
use futures::future::try_join;
use std::sync::Arc;

impl PageRenderer {
    pub async fn home_page(ctx: Arc<ApplicationContext>) -> Result<()> {
        let posts = ctx
            .client
            .vc
            .get_post_by_parent_post("".to_string(), 1)
            .await?;
        let posts = posts
            .iter()
            .map(|p| PostComponent::new(p.clone()) as Box<dyn Component>)
            .collect::<Vec<Box<dyn Component>>>();
        let nav_bar = NavBar::new(ctx.session.clone());
        let mut post_creator: Option<Box<dyn Component>> = None;
        if ctx.session.is_some() {
            post_creator = Some(PostCreator::new());
        }
        let comp = BlawgdHTMLDoc::new(HomePage::new(
            nav_bar,
            post_creator,
            posts.into_boxed_slice(),
        ));

        let body = ctx.window.document()?.body()?;
        body.set_inner_html(&comp.to_html());
        comp.register_events(ctx)?;

        Ok(())
    }
}
