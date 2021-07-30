use crate::blawgd_client::query_client::QueryClient as BlawgdQueryClient;
use crate::blawgd_client::GetPostsByParentPostRequest;
use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::edit_profile_page::EditProfilePage;
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

    let account_info = util::get_account_info_from_storage(&storage);
    if account_info.is_none() {
        window.location().replace(util::HOST_NAME);
    }

    let account_info_comp = AccountInfoComp::new(account_info.clone().unwrap());
    let nav_bar = NavBar::new(account_info.clone());
    let comp = BlawgdHTMLDoc::new(EditProfilePage::new(nav_bar, account_info_comp));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    register_event_listeners(&document)
}

fn register_event_listeners(document: &web_sys::Document) {
    let preview_button = document
        .get_element_by_id("preview-button")
        .expect("preview-button element not found");

    events::EventListener::new(&preview_button, "click", move |_| {}).forget();
}
