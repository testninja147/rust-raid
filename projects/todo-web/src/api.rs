use actix_web::{HttpResponse, Responder};

pub(crate) struct Todo;

impl Todo {
    pub(crate) async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    pub(crate) async fn echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    pub(crate) async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }
}
