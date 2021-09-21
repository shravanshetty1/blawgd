use crate::dom::Document;
use crate::host::Host;
use crate::storage::Store;
use crate::{
    clients::verification_client::VerificationClient,
    components::blawgd_html::BlawgdHTMLDoc,
    components::home_page::HomePage,
    components::nav_bar::NavBar,
    components::post::PostComponent,
    components::post_creator::PostCreator,
    components::Component,
    dom,
    state::{get_state, set_state},
    util,
    util::register_post_event_listener,
};
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle(
    host: Host,
    store: Store,
    window: dom::Window,
    cl: VerificationClient,
) -> Result<()> {
    let document = window.document()?;

    // TODO make this concurrent
    let posts = cl.get_post_by_parent_post("".to_string(), 1).await?;
    let account_info = store.get_session_account_info(cl.clone()).await.ok();

    let post_components = posts
        .iter()
        .map(|p| PostComponent::new(p.clone()) as Box<dyn Component>)
        .collect::<Vec<Box<dyn Component>>>();
    let nav_bar = NavBar::new(account_info.clone());
    let mut post_creator: Option<Box<dyn Component>> = None;
    if account_info.is_some() {
        post_creator = Some(PostCreator::new());
    }
    let comp = BlawgdHTMLDoc::new(HomePage::new(
        nav_bar,
        post_creator,
        post_components.into_boxed_slice(),
    ));

    let body = document.body()?;
    body.set_inner_html(&comp.to_html());

    if account_info.is_some() {
        let address = account_info
            .clone()
            .ok_or(anyhow!("account info is empty"))?
            .address;
        let client = grpc_web_client::Client::new(host.grpc_endpoint().into());
        let wallet = store.get_wallet()?;
        for post in posts {
            register_post_event_listener(wallet.clone(), client.clone(), address.clone(), post)
        }
        register_event_listeners(store, host, document.clone(), cl.clone())?;
    }

    Ok(())
}

fn register_event_listeners(
    store: Store,
    host: Host,
    document: Document,
    cl: VerificationClient,
) -> Result<()> {
    let post_creator_button = document.get_element_by_id("post-creator-button")?.inner();
    events::EventListener::new(&post_creator_button, "click", move |_| {
        let store = store.clone();
        let host = host.clone();
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
                creator: store.get_application_data().unwrap().address,
                content: post_content,
                parent_post: "".to_string(),
            };

            let wallet = store.get_wallet().unwrap();
            let client = grpc_web_client::Client::new(host.grpc_endpoint());
            let resp = util::broadcast_tx(
                &wallet,
                client,
                util::MSG_TYPE_CREATE_POST,
                msg,
                BroadcastMode::Block as i32,
            )
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

    Ok(())
}
