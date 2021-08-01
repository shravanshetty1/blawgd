use crate::blawgd_client::{GetAccountInfoRequest, GetPostsByAccountRequest};
use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::profile_page::ProfilePage;
use crate::components::Component;
use crate::util;

pub async fn handle() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());

    let url: String = window.location().href().unwrap();
    let address = url
        .as_str()
        .strip_prefix("http://localhost:2341/profile/")
        .unwrap();

    let logged_in_data = util::get_stored_data(&storage);
    let mut show_edit_button = false;
    if logged_in_data.is_some() {
        if address == logged_in_data.unwrap().address {
            show_edit_button = true;
        }
    }

    let logged_in_account_info = util::get_session_account_info(&storage, client.clone());
    let account_info = util::get_account_info(client.clone(), address.clone().into());
    let posts_resp = super::blawgd_client::query_client::QueryClient::new(client)
        .get_posts_by_account(GetPostsByAccountRequest {
            address: address.clone().into(),
            index: 0,
        })
        .await
        .unwrap();
    let mut posts: Vec<Box<dyn Component>> = Vec::new();
    for post in &posts_resp.get_ref().posts {
        let post_comp = Post::new(post.clone());
        posts.push(post_comp)
    }

    let nav_bar = NavBar::new(logged_in_account_info.await.clone());
    let comp = BlawgdHTMLDoc::new(ProfilePage::new(
        nav_bar,
        AccountInfoComp::new(account_info.await.clone()),
        show_edit_button,
        posts.into_boxed_slice(),
    ));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());
}
