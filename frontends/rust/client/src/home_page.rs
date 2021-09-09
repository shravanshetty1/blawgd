use crate::blawgd_client::query_client::QueryClient as BlawgdQueryClient;
use crate::blawgd_client::verification_client::VerificationClient;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::PostComponent;
use crate::components::post_creator::PostCreator;
use crate::components::Component;
use crate::state::{get_state, set_state, State};
use crate::util;
use anyhow::Result;
use anyhow::{anyhow, Context};
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle(cl: VerificationClient) -> Result<()> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let storage = window.local_storage().unwrap().unwrap();

    let account_info = util::get_session_account_info(&storage, cl.clone()).await;
    let posts = cl
        .get_post_by_parent_post("".to_string(), 1)
        .await
        .context("failed to get posts for home")?;
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
        register_event_listeners(document.clone(), cl.clone());
    }

    Ok(())
}

fn register_event_listeners(document: web_sys::Document, cl: VerificationClient) {
    let post_creator_button = document
        .clone()
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
            let resp = util::broadcast_tx(&wallet, client, util::MSG_TYPE_CREATE_POST, msg)
                .await
                .into_inner();

            util::console_log(resp.tx_response.unwrap().raw_log.as_str());

            window.location().reload();
        });
    })
    .forget();

    let window = web_sys::window().unwrap();
    events::EventListener::new(&window, "scroll", move |_| {
        let cl = cl.clone();
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
                    .get_post_by_parent_post("".to_string(), state.page.clone() as u64)
                    .await
                    .unwrap();
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
