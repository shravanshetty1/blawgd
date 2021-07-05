mod components;
use crate::components::Component;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn home(req: HttpRequest) -> impl Responder {
    let home_page_component =
        components::blawgd_html::BlawgdHTMLDoc::new(components::home_page::HomePage::new());
    actix_web::HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(actix_web::body::Body::from(home_page_component.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/static", "../dist").show_files_listing())
            .route("/", web::get().to(home))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
