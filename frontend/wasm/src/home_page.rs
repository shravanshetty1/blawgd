use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::post_creator::PostCreator;
use crate::components::Component;

pub fn handle(window: &web_sys::Window) {
    let document = window.document().expect("document missing");

    let nav_bar = NavBar::new();
    let post_creator = PostCreator::new();
    let post = Post::new();
    let comp = BlawgdHTMLDoc::new(HomePage::new(nav_bar, post_creator, Box::new([post])));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());
}
