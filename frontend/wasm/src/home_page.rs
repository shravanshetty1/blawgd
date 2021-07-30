use crate::blawgd_client::query_client::QueryClient as BlawgdQueryClient;
use crate::blawgd_client::GetPostsRequest;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::post_creator::PostCreator;
use crate::components::Component;
use crate::util;
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle() {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("document missing");
    let storage = window
        .local_storage()
        .expect("storage object missing")
        .unwrap();

    let client = grpc_web_client::Client::new("http://localhost:9091".into());
    let posts_resp = BlawgdQueryClient::new(client)
        .get_posts(GetPostsRequest { index: 0 })
        .await
        .unwrap();
    let mut posts: Vec<Box<dyn Component>> = Vec::new();
    for post in &posts_resp.get_ref().posts {
        posts.push(Post::new(post.clone()))
    }

    let account_info = util::get_account_info_from_storage(&storage);
    let nav_bar = NavBar::new(account_info.clone());
    let post_creator = PostCreator::new();
    let comp = BlawgdHTMLDoc::new(HomePage::new(
        nav_bar,
        post_creator,
        posts.into_boxed_slice(),
    ));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    register_event_listeners(&document)
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

            let account_info = util::get_account_info_from_storage(&storage);
            let post_content: String = document
                .get_element_by_id("post-creator-input")
                .expect("post-creator-input element not found")
                .dyn_ref::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .value();
            let msg = super::blawgd_client::MsgCreatePost {
                creator: account_info.unwrap().address.clone(),
                content: post_content,
                parent_post: "".to_string(),
                metadata: "".to_string(),
            };

            let wallet = util::get_wallet(&storage).unwrap();
            let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
            util::broadcast_tx(&wallet, client, util::MSG_TYPE_CREATE_POST, msg).await;
            window.location().reload();
        });
    })
    .forget();
}
