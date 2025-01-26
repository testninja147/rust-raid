use std::sync::Mutex;

use actix_files as fs;
use actix_web::{
    middleware::DefaultHeaders,
    web::{self},
    App, HttpServer,
};

mod api;
mod todo;

use api::{create, delete, list, retrieve, update};
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
                web::scope("/api")
                    .wrap(DefaultHeaders::new().add(("Content-Type", "application/json")))
                    .service(
                        web::scope("/todo")
                            .route("/", web::get().to(list))
                            .route("/", web::post().to(create))
                            .route("/{id}/", web::get().to(retrieve))
                            .route("/{id}/", web::patch().to(update))
                            .route("/{id}/", web::delete().to(delete)),
                    ),
            )
            // serve static files and index.html file
            .service(fs::Files::new("/static", "projects/todo-web/static").show_files_listing())
            .service(fs::Files::new("/", "projects/todo-web/").index_file("index.html"))

        // use paths below instead if you execute `cargo run` command from `todo-web` directory instead of running
        // cargo run --bin todo-web from the project directory

        // .service(fs::Files::new("/static", "static").show_files_listing())
        // .service(fs::Files::new("/", ".").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
