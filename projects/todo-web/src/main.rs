use actix_web::{web, App, HttpServer};
mod api;

use api::Todo;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Prefix all services with /api for API endpoints
            .service(
                web::scope("/api")
                    .route("/echo", web::get().to(Todo::echo))
                    .route("/hey", web::get().to(Todo::manual_hello))
                    .route("/", web::get().to(Todo::hello)),
            )
            .route("/", web::get().to(Todo::hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
