use gloo::events;
use wasm_bindgen::JsCast;

use crate::blawgd_client::verification_client::VerificationClient;
use crate::{
    blawgd_client::query_client::QueryClient as BlawgdQueryClient,
    components::blawgd_html::BlawgdHTMLDoc, components::home_page::HomePage,
    components::nav_bar::NavBar, components::post::PostComponent,
    components::post_creator::PostCreator, components::Component, util,
};
use anyhow::anyhow;
use anyhow::Result;

pub async fn handle(cl: VerificationClient) -> Result<()> {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("document missing");
    let storage = window
        .local_storage()
        .expect("storage object missing")
        .unwrap();

    let account_info = util::get_session_account_info(&storage, cl.clone()).await;

    let posts = cl
        .get_timeline(
            account_info
                .clone()
                .ok_or(anyhow!("user not logged in"))?
                .address
                .clone(),
        )
        .await?;
    let mut boxed_posts: Vec<Box<dyn Component>> = Vec::new();
    for post in posts {
        boxed_posts.push(PostComponent::new(post))
    }
    let nav_bar = NavBar::new(account_info.clone());
    let mut post_creator: Option<Box<dyn Component>> = None;
    if account_info.is_some() {
        post_creator = Some(PostCreator::new());
    }
    let comp = BlawgdHTMLDoc::new(HomePage::new(
        nav_bar,
        post_creator,
        boxed_posts.into_boxed_slice(),
    ));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    if account_info.is_some() {
        register_event_listeners(&document)
    };
    Ok(())
}

fn register_event_listeners(document: &web_sys::Document) {
    let post_creator_button = document
        .get_element_by_id("post-creator-button")
        .expect("post-creator-button element not found");

    events::EventListener::new(&post_creator_button, "click", move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().expect("document missing");
            let storage = window.local_storage().unwrap().unwrap();

            let post_content: String = document
                .get_element_by_id("post-creator-input")
                .expect("post-creator-input element not found")
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .value();
            let msg = super::blawgd_client::MsgCreatePost {
                creator: util::get_stored_data(&storage).unwrap().address,
                content: post_content,
                parent_post: "".to_string(),
            };

            let wallet = util::get_wallet(&storage).unwrap();
            let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
            util::broadcast_tx(&wallet, client, util::MSG_TYPE_CREATE_POST, msg).await;
            window.location().reload();
        });
    })
    .forget();
}
