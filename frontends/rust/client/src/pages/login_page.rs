use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::login_page::LoginPage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageRenderer;
use anyhow::Result;
use std::sync::Arc;

impl PageRenderer {
    pub async fn login_page(ctx: Arc<ApplicationContext>) -> Result<()> {
        let mut account_info_comp: Option<Box<dyn Component>> = None;
        if ctx.session.is_some() {
            account_info_comp = Some(AccountInfoComp::new(ctx.session.clone().unwrap()))
        }
        let nav_bar = NavBar::new(ctx.session.clone());
        let comp = BlawgdHTMLDoc::new(LoginPage::new(nav_bar, account_info_comp));

        let body = ctx.window.document()?.body()?;
        body.set_inner_html(&comp.to_html());
        comp.register_events(ctx);

        Ok(())
    }
}
