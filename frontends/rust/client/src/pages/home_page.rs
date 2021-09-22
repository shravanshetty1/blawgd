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
        let posts_resp = ctx.client.vc.get_post_by_parent_post("".to_string(), 1);
        let account_info_resp = ctx.store.get_session_account_info(ctx.client.vc.clone());
        let (posts, account_info) = try_join(posts_resp, account_info_resp).await?;
        let account_info = Some(account_info);

        let post_components = posts
            .iter()
            .map(|p| PostComponent::new(p.clone()) as Box<dyn Component>)
            .collect::<Vec<Box<dyn Component>>>();
        let nav_bar = NavBar::new(account_info.clone());
        let mut post_creator: Option<Box<dyn Component>> = None;
        if account_info.is_some() {
            post_creator = Some(PostCreator::new());
        }
        let comp = BlawgdHTMLDoc::new(HomePage::new(
            nav_bar,
            post_creator,
            post_components.into_boxed_slice(),
        ));

        let body = ctx.window.document()?.body()?;
        body.set_inner_html(&comp.to_html());
        comp.register_events(ctx)?;

        Ok(())
    }
}
