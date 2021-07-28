use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::post_creator::PostCreator;
use crate::components::Component;

pub fn handle(window: &web_sys::Window) {
    let document = window.document().expect("document missing");
    let storage = window
        .local_storage()
        .expect("storage object missing")
        .unwrap();

    let account_info = super::util::get_account_info_from_storage(&storage);
    let nav_bar = NavBar::new(account_info);
    let post_creator = PostCreator::new();
    let post = Post::new();
    let comp = BlawgdHTMLDoc::new(HomePage::new(nav_bar, post_creator, Box::new([post])));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());
}
