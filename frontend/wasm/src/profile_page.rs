use crate::blawgd_client::{GetAccountInfoRequest, GetPostsByAccountRequest};
use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::profile_page::ProfilePage;
use crate::components::Component;
use crate::util;
use crate::util::StoredData;
use gloo::events;
use tonic::codegen::Future;

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
    let mut show_button = false;
    if logged_in_data.is_some() {
        show_button = true;
        if address == logged_in_data.as_ref().unwrap().address {
            show_edit_button = true;
        }
    }

    let is_following = util::is_following(
        client.clone(),
        logged_in_data.as_ref().unwrap().address.clone(),
        address.into(),
    );
    let logged_in_account_info = util::get_session_account_info(&storage, client.clone());
    let account_info = util::get_account_info(client.clone(), address.clone().into());
    let posts_resp = super::blawgd_client::query_client::QueryClient::new(client.clone())
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
    let profile_page = ProfilePage::new(
        nav_bar,
        AccountInfoComp::new(account_info.await.clone()),
        show_edit_button,
        posts.into_boxed_slice(),
        is_following.await,
        show_button,
    );

    let comp = BlawgdHTMLDoc::new(profile_page);

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    if logged_in_data.is_some() && !show_edit_button {
        register_event_listeners(&document, address.into());
    }
}

fn register_event_listeners(document: &web_sys::Document, address: String) {
    let follow_toggle = document.get_element_by_id("follow-toggle").unwrap();

    events::EventListener::new(&follow_toggle, "click", move |_| {
        let address = address.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());

            let session_address = util::get_stored_data(&storage).unwrap().address;
            if util::is_following(client.clone(), session_address.clone(), address.clone()).await {
                let msg = Box::new(crate::blawgd_client::MsgStopFollow {
                    creator: session_address.clone(),
                    address: address.clone(),
                });
                let msg_type = util::MSG_TYPE_STOP_FOLLOW;
                util::broadcast_tx(
                    &util::get_wallet(&storage).unwrap(),
                    client.clone(),
                    msg_type,
                    msg,
                )
                .await;
            } else {
                let msg = Box::new(crate::blawgd_client::MsgFollow {
                    creator: session_address.clone(),
                    address: address.clone(),
                });
                let msg_type = util::MSG_TYPE_FOLLOW;
                util::broadcast_tx(
                    &util::get_wallet(&storage).unwrap(),
                    client.clone(),
                    msg_type,
                    msg,
                )
                .await;
            }

            window.location().reload();
        });
    })
    .forget();
}
