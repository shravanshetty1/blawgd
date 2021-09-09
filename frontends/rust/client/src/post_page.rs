use crate::blawgd_client::query_client::QueryClient as BlawgdQueryClient;
use crate::blawgd_client::verification_client::VerificationClient;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::nav_bar::NavBar;
use crate::components::post::PostComponent;
use crate::components::post_creator::PostCreator;
use crate::components::post_page::PostPage;
use crate::components::Component;
use crate::util;
use anyhow::Result;
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle(cl: VerificationClient) -> Result<()> {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("document missing");
    let storage = window
        .local_storage()
        .expect("storage object missing")
        .unwrap();

    let url: String = window.location().href().unwrap();
    let post_id = url
        .as_str()
        .strip_prefix(format!("{}/post/", util::HOST_NAME).as_str())
        .unwrap()
        .to_string();

    let account_info = util::get_session_account_info(&storage, cl.clone()).await;
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

    register_event_listeners(post_id.to_string(), &document);

    Ok(())
}

fn register_event_listeners(main_post_id: String, document: &web_sys::Document) {
    let post_creator_button = document
        .get_element_by_id("post-creator-button")
        .expect("post-creator-button element not found");

    events::EventListener::new(&post_creator_button, "click", move |_| {
        let main_post_id: String = main_post_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().expect("document missing");
            let storage = window.local_storage().unwrap().unwrap();

            let address: String = util::get_stored_data(&storage).unwrap().address;
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

            let wallet = util::get_wallet(&storage).unwrap();
            let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
            util::broadcast_tx(&wallet, client, util::MSG_TYPE_CREATE_POST, msg).await;
            window.location().reload();
        });
    })
    .forget();
}
