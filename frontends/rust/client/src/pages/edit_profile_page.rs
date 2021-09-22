use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::edit_profile_page::EditProfilePage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageRenderer;
use anyhow::Result;
use std::sync::Arc;

impl PageRenderer {
    pub async fn edit_profile_page(ctx: Arc<ApplicationContext>) -> Result<()> {
        let session = ctx.session.clone();

        let account_info_comp = AccountInfoComp::new(session.clone().unwrap());
        let nav_bar = NavBar::new(session.clone());
        let comp = BlawgdHTMLDoc::new(EditProfilePage::new(nav_bar, account_info_comp));

        let body = ctx.window.document()?.body()?;
        body.set_inner_html(&comp.to_html());
        comp.register_events(ctx);

        Ok(())
    }
}
