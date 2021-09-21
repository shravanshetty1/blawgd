use crate::clients::verification_client::VerificationClient;
use crate::host::Host;
use crate::storage::Store;
use crate::{
    components::{
        blawgd_html::BlawgdHTMLDoc, nav_bar::NavBar, post::PostComponent,
        post_creator::PostCreator, post_page::PostPage, Component,
    },
    state::{get_state, set_state},
    util,
};
use anyhow::Result;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle(store: Store, host: Host, cl: VerificationClient) -> Result<()> {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("document missing");

    let url: String = window.location().href().unwrap();
    let post_id = url
        .as_str()
        .strip_prefix(format!("{}/post/", host.endpoint()).as_str())
        .unwrap()
        .to_string();

    let account_info = store.get_session_account_info(cl.clone()).await.ok();
    let posts = cl.get_post_by_parent_post(post_id.clone(), 1).await?;
    let mut boxed_posts: Vec<Box<dyn Component>> = Vec::new();
    for post in posts {
        boxed_posts.push(PostComponent::new(post))
    }

    let main_post = cl.get_post(post_id.clone()).await?;
    let mut main_post = PostComponent::new(main_post);
    main_post.as_mut().focus();

    let nav_bar = NavBar::new(account_info.clone());
    let mut post_creator_component: Option<Box<dyn Component>> = None;
    if account_info.is_some() {
        let mut post_creator = PostCreator::new();
        post_creator.as_mut().set_button_text("Reply");
        post_creator_component = Some(post_creator);
    }

    let comp = BlawgdHTMLDoc::new(PostPage::new(
        nav_bar,
        main_post,
        post_creator_component,
        boxed_posts.into_boxed_slice(),
    ));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    register_event_listeners(store, host, post_id.to_string(), &document, cl.clone());

    Ok(())
}

fn register_event_listeners(
    store: Store,
    host: Host,
    main_post_id: String,
    document: &web_sys::Document,
    cl: VerificationClient,
) {
    let post_creator_button = document
        .get_element_by_id("post-creator-button")
        .expect("post-creator-button element not found");

    let main_post_id1: String = main_post_id.clone();
    events::EventListener::new(&post_creator_button, "click", move |_| {
        let main_post_id: String = main_post_id1.clone();
        let store = store.clone();
        let host = host.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().expect("document missing");

            let address: String = store.get_application_data().unwrap().address;
            let post_content: String = document
                .get_element_by_id("post-creator-input")
                .expect("post-creator-input element not found")
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .value();
            let msg = super::blawgd_client::MsgCreatePost {
                creator: address,
                content: post_content,
                parent_post: main_post_id,
            };

            let wallet = store.get_wallet().unwrap();
            let client = grpc_web_client::Client::new(host.grpc_endpoint());
            util::broadcast_tx(
                &wallet,
                client,
                util::MSG_TYPE_CREATE_POST,
                msg,
                BroadcastMode::Block as i32,
            )
            .await;
            window.location().reload();
        });
    })
    .forget();

    let window = web_sys::window().unwrap();
    events::EventListener::new(&window, "scroll", move |_| {
        let cl = cl.clone();
        let main_post_id = main_post_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().expect("document missing");
            let doc = document.document_element().unwrap();
            let scroll_top: i32 = doc.scroll_top();
            let scroll_height: i32 = doc.scroll_height();
            let client_height: i32 = doc.client_height();
            let main_column = document
                .clone()
                .get_element_by_id("main-column")
                .expect("post-creator-button element not found");

            if scroll_top + client_height >= scroll_height {
                let mut state = get_state();
                state.page += 1;

                let posts = cl
                    .get_post_by_parent_post(main_post_id.clone(), state.page.clone() as u64)
                    .await
                    .unwrap();
                if posts.len() == 0 {
                    return;
                }
                let mut posts_html: String = String::new();
                for post in posts {
                    posts_html = format!("{}{}", posts_html, PostComponent::new(post).to_html());
                }

                main_column.insert_adjacent_html("beforeend", posts_html.as_str());

                set_state(state.clone());
                util::console_log(format!("{}", state.page).as_str());
            }
        });
    })
    .forget();
}
