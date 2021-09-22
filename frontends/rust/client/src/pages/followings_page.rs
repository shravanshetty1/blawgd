use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::followings_page::FollowingsPage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageRenderer;
use anyhow::anyhow;
use anyhow::Result;
use std::sync::Arc;

impl PageRenderer {
    pub async fn followings_page(ctx: Arc<ApplicationContext>) -> Result<()> {
        let document = ctx.window.document()?;
        let url = ctx.window.location().href()?;
        let address = url
            .as_str()
            .strip_prefix(format!("{}/followings/", ctx.host.endpoint()).as_str())
            .ok_or(anyhow!("could not get address from url {}", url))?;

        let followings = ctx
            .client
            .vc
            .get_following_list(address.clone().parse()?)
            .await?;
        let followings_account_info = ctx.client.vc.get_account_infos(followings).await?;

        let nav_bar = NavBar::new(ctx.session.clone());
        let comp = BlawgdHTMLDoc::new(FollowingsPage::new(nav_bar, followings_account_info));

        let body = document.body()?;
        body.set_inner_html(&comp.to_html());
        comp.register_events(ctx);

        Ok(())
    }
}
