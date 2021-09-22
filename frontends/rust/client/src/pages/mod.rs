mod edit_profile_page;
mod followings_page;
mod home_page;
mod login_page;
mod post_page;
mod profile_page;
mod timeline_page;
use crate::context::ApplicationContext;
use anyhow::anyhow;
use anyhow::Result;
use prost::alloc::sync::Arc;

pub struct PageRenderer {
    ctx: Arc<ApplicationContext>,
}

impl PageRenderer {
    pub fn new(ctx: Arc<ApplicationContext>) -> PageRenderer {
        PageRenderer { ctx }
    }

    pub async fn render(&self, url: &str) -> Result<()> {
        let ctx = self.ctx.clone();

        let url_path = url
            .strip_prefix(format!("{}/", ctx.host.endpoint()).as_str())
            .ok_or(anyhow!("could not stip prefix of {}", url))?;

        match url_path {
            url if url.starts_with("followings") => PageRenderer::followings_page(ctx).await,
            // url if url.starts_with("post") => post_page::handle(Store, host, cl).await,
            url if url.starts_with("edit-profile") => PageRenderer::edit_profile_page(ctx).await,
            // url if url.starts_with("timeline") => timeline_page::handle(host, Store, cl).await,
            // url if url.starts_with("profile") => profile_page::handle(Store, host, cl).await,
            url if url.starts_with("login") => PageRenderer::login_page(ctx).await,
            _ => PageRenderer::home_page(ctx).await,
        }?;

        Ok(())
    }
}
