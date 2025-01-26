use actix_web::{web, HttpResponse, Responder};

use crate::ApplicationState;

pub(crate) async fn list(data: web::Data<ApplicationState>) -> impl Responder {
    let items = (&data.todo_list.lock().unwrap().items).clone();
    let todo_list = Vec::from_iter(items.values());
    let str = serde_json::to_string(&todo_list).unwrap();
    HttpResponse::Ok().body(str)
}
pub(crate) async fn create(data: web::Data<ApplicationState>) -> impl Responder {
    // let todo_list = data.todo_list.lock().unwrap();
    HttpResponse::Ok().body("[]")
}
pub(crate) async fn retrieve(
    data: web::Data<ApplicationState>,
    path: web::Path<(usize,)>,
) -> impl Responder {
    let items = &data.todo_list.lock().unwrap().items;
    return match (items).get(&path.into_inner().0) {
        Some(item) => {
            let str = serde_json::to_string(item).unwrap();
            HttpResponse::Ok().body(str)
        }
        None => HttpResponse::NotFound().body("{\"detail\": \"Not found\"}"),
    };
}
pub(crate) async fn update(data: web::Data<ApplicationState>) -> impl Responder {
    let todo_list = data.todo_list.lock().unwrap();
    HttpResponse::Ok().body("[]")
}
pub(crate) async fn delete(data: web::Data<ApplicationState>) -> impl Responder {
    let todo_list = data.todo_list.lock().unwrap();
    println!("{:?}", *todo_list);
    HttpResponse::Ok().body("[]")
}
