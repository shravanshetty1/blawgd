mod components;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use components::blawgd_html::BlawgdHTMLDoc;
use components::home_page::HomePage;
use components::Component;

async fn home(req: HttpRequest) -> impl Responder {
    let home_page_component = BlawgdHTMLDoc::new(HomePage::new());
    actix_web::HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(actix_web::body::Body::from(home_page_component.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting frontend on port 8080");
    HttpServer::new(|| {
        App::new().service(
            actix_files::Files::new("/", "../dist")
                .show_files_listing()
                .index_file("index.html"),
        )
        // .route("/", web::get().to(home))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
