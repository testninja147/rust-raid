use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
mod api;
mod todo;

use api::{delete, list, retrieve, update};
use todo::TodoList;

struct ApplicationState {
    todo_list: Mutex<TodoList>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(ApplicationState {
        todo_list: Mutex::new(TodoList::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // Prefix all services with /api for API endpoints
            .service(
                web::scope("/api/todo")
                    .route("/", web::get().to(list))
                    .route("/{id}/", web::get().to(retrieve))
                    .route("/{id}/", web::patch().to(update))
                    .route("/{id}/", web::delete().to(delete)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
