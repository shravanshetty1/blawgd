use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::PostComponent;
use crate::components::post_creator::PostCreator;
use crate::components::timeline_page::TimelinePage;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageRenderer;
use anyhow::anyhow;
use anyhow::Result;
use std::sync::Arc;

impl PageRenderer {
    pub async fn timeline_page(ctx: Arc<ApplicationContext>) -> Result<()> {
        let posts = ctx
            .client
            .vc
            .get_timeline(
                ctx.session
                    .clone()
                    .ok_or(anyhow!("user not logged in"))?
                    .address
                    .clone(),
                1,
            )
            .await?;
        let posts = posts
            .iter()
            .map(|p| PostComponent::new(p.clone()) as Box<dyn Component>)
            .collect::<Vec<Box<dyn Component>>>();
        let nav_bar = NavBar::new(ctx.session.clone());
        let mut post_creator: Option<Box<dyn Component>> = None;
        if ctx.session.is_some() {
            post_creator = Some(PostCreator::new("".to_string()));
        }
        let comp = BlawgdHTMLDoc::new(TimelinePage::new(
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
