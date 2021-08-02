use crate::blawgd_client;
use crate::blawgd_client::{AccountInfoView, GetAccountInfoRequest, GetPostsByAccountRequest};
use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::followings_page::FollowingsPage;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::profile_page::ProfilePage;
use crate::components::Component;
use crate::util;
use crate::util::StoredData;
use gloo::events;

pub async fn handle() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());

    let url: String = window.location().href().unwrap();
    let address = url
        .as_str()
        .strip_prefix(format!("{}/followings/", util::HOST_NAME))
        .unwrap();

    let account_info = util::get_session_account_info(&storage, client.clone());
    let followings = blawgd_client::query_client::QueryClient::new(client.clone())
        .clone()
        .get_followings(blawgd_client::GetFollowingsRequest {
            address: address.clone().into(),
        })
        .await
        .unwrap()
        .get_ref()
        .addresses
        .clone();

    let mut followings_account_info_futures: Vec<_> = vec![];
    for following in followings {
        followings_account_info_futures.push(util::get_account_info(client.clone(), following))
    }

    let mut followings_account_info: Vec<_> = vec![];
    for future in followings_account_info_futures {
        followings_account_info.push(future.await)
    }

    let nav_bar = NavBar::new(account_info.await.clone());
    let comp = BlawgdHTMLDoc::new(FollowingsPage::new(nav_bar, followings_account_info));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());
}
