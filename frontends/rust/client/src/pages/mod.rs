use crate::clients::verification_client::VerificationClient;
use crate::dom::Window;
use crate::host::Host;
use crate::storage::Store;
mod edit_profile_page;
mod followings_page;
mod home_page;
mod login_page;
mod post_page;
mod profile_page;
mod timeline_page;
use anyhow::anyhow;
use anyhow::Result;

pub struct PageRenderer {
    host: Host,
    store: Store,
    window: Window,
    client: VerificationClient,
    // TODO remove this
    grpc_client: grpc_web_client::Client,
}

impl PageRenderer {
    pub fn new(
        host: Host,
        store: Store,
        window: Window,
        cl: VerificationClient,
        grpc_client: grpc_web_client::Client,
    ) -> PageRenderer {
        PageRenderer {
            host,
            store,
            window,
            client: cl,
            grpc_client,
        }
    }

    pub async fn render(&self, url: &str) -> Result<()> {
        let url_path = url
            .strip_prefix(format!("{}/", self.host.endpoint()).as_str())
            .ok_or(anyhow!("could not stip prefix of {}", url))?;

        match url_path {
            // url if url.starts_with("followings") => followings_page::handle(Store, host, cl).await,
            // url if url.starts_with("post") => post_page::handle(Store, host, cl).await,
            // url if url.starts_with("edit-profile") => {
            //     edit_profile_page::handle(Store, host, cl).await
            // }
            // url if url.starts_with("timeline") => timeline_page::handle(host, Store, cl).await,
            // url if url.starts_with("profile") => profile_page::handle(Store, host, cl).await,
            // url if url.starts_with("login") => login_page::handle(Store, host, cl).await,
            _ => self.home_page().await,
        }?;

        Ok(())
    }
}
