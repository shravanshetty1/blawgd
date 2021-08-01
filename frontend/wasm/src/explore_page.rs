use crate::blawgd_client::query_client::QueryClient as BlawgdQueryClient;
use crate::blawgd_client::GetPostsByParentPostRequest;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::explore_page::ExplorePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::Component;
use crate::util;

pub async fn handle() {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("document missing");
    let storage = window
        .local_storage()
        .expect("storage object missing")
        .unwrap();
    let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());

    let account_info = util::get_session_account_info(&storage, client.clone());
    let posts_resp = BlawgdQueryClient::new(client)
        .get_posts_by_parent_post(GetPostsByParentPostRequest {
            parent_post: "".to_string(),
            index: 0,
        })
        .await
        .unwrap();
    let mut posts: Vec<Box<dyn Component>> = Vec::new();
    for post in &posts_resp.get_ref().posts {
        posts.push(Post::new(post.clone()))
    }

    let nav_bar = NavBar::new(account_info.await.clone());
    let comp = BlawgdHTMLDoc::new(ExplorePage::new(nav_bar, posts.into_boxed_slice()));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());
}
