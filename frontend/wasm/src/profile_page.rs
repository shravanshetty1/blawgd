use crate::blawgd_client::{GetAccountInfoRequest, GetPostsRequest};
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::profile_page::ProfilePage;
use crate::components::Component;

pub fn handle() {
    wasm_bindgen_futures::spawn_local(async move {
        let window = web_sys::window().unwrap();
        let document = window.document().expect("document missing");
        let storage = window
            .local_storage()
            .expect("storage object missing")
            .unwrap();
        let url: String = window.location().href().unwrap();
        let address = url
            .as_str()
            .strip_prefix("http://localhost:2341/profile/")
            .unwrap();

        let logged_in_account = super::util::get_account_info_from_storage(&storage);

        let mut show_edit_button = true;
        let mut account_info = logged_in_account.clone().unwrap();
        let client = grpc_web_client::Client::new("http://localhost:9091".into());
        if address != logged_in_account.unwrap().address {
            show_edit_button = false;
            let acc_resp = super::blawgd_client::query_client::QueryClient::new(client.clone())
                .get_account_info(GetAccountInfoRequest {
                    address: address.into(),
                })
                .await
                .unwrap();
            account_info = acc_resp.get_ref().account_info.clone().unwrap();

            if account_info.address.is_empty() {
                account_info.address = address.into();
            }
        }

        let posts_resp = super::blawgd_client::query_client::QueryClient::new(client)
            .get_posts(GetPostsRequest { index: 0 })
            .await
            .unwrap();
        let mut posts: Vec<Box<dyn Component>> = Vec::new();
        for post in &posts_resp.get_ref().posts {
            let post_comp = Post::new(post.clone());
            posts.push(post_comp)
        }

        let nav_bar = NavBar::new(Some(account_info.clone()));
        let comp = BlawgdHTMLDoc::new(ProfilePage::new(
            nav_bar,
            Some(account_info.clone()),
            show_edit_button,
            posts.into_boxed_slice(),
        ));

        let body = document.body().expect("body missing");
        body.set_inner_html(&comp.to_html());
    });
}
