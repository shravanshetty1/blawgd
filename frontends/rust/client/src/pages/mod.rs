mod edit_profile_page;
mod followings_page;
mod home_page;
mod login_page;
mod post_page;
mod profile_page;
mod timeline_page;
use crate::components::Component;
use crate::context::ApplicationContext;
use anyhow::anyhow;
use anyhow::Result;
use prost::alloc::sync::Arc;

pub struct PageRenderer {
    ctx: Arc<ApplicationContext>,
}

pub struct PageBuilder;

impl PageRenderer {
    pub fn new(ctx: Arc<ApplicationContext>) -> PageRenderer {
        PageRenderer { ctx }
    }

    pub async fn render(&self, url: &str) -> Result<()> {
        let ctx = self.ctx.clone();

        let url_path = url
            .strip_prefix(format!("{}/", ctx.host.endpoint()).as_str())
            .ok_or(anyhow!("could not stip prefix of {}", url))?;

        // TODO return components and render outside
        let page: Box<dyn Component> = match url_path {
            url if url.starts_with("followings") => PageBuilder::followings_page(ctx.clone()).await,
            url if url.starts_with("post") => PageBuilder::post_page(ctx.clone()).await,
            url if url.starts_with("edit-profile") => {
                PageBuilder::edit_profile_page(ctx.clone()).await
            }
            url if url.starts_with("timeline") => PageBuilder::timeline_page(ctx.clone()).await,
            url if url.starts_with("profile") => PageBuilder::profile_page(ctx.clone()).await,
            url if url.starts_with("login") => PageBuilder::login_page(ctx.clone()).await,
            _ => PageBuilder::home_page(ctx.clone()).await,
        }?;

        let body = ctx.window.document()?.get_element_by_id("body")?;
        body.set_inner_html(&page.to_html());
        page.register_events(ctx)?;

        Ok(())
    }
}
