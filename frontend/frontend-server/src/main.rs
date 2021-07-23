mod components;
use crate::components::post::Post;
use crate::components::post_creator::PostCreator;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use components::blawgd_html::BlawgdHTMLDoc;
use components::home_page::HomePage;
use components::nav_bar::NavBar;
use components::Component;

async fn home(req: HttpRequest) -> impl Responder {
    let nav_bar = NavBar::new();
    let post_creator = PostCreator::new();
    let post = Post::new();
    let home_page_component =
        BlawgdHTMLDoc::new(HomePage::new(nav_bar, post_creator, Box::new([post])));
    actix_web::HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(actix_web::body::Body::from(home_page_component.to_html()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting frontend on port 8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .service(actix_files::Files::new("/", "../dist").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
