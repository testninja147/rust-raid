use actix_web::{web, HttpResponse, Responder};

use crate::{todo::TodoCreateUpdate, ApplicationState};

pub(crate) async fn list(data: web::Data<ApplicationState>) -> impl Responder {
    let items = &data.todo_list.lock().unwrap().items;
    let todo_list = Vec::from_iter(items.values());
    let str = serde_json::to_string(&todo_list).unwrap();
    HttpResponse::Ok().body(str)
}

pub(crate) async fn create(
    data: web::Data<ApplicationState>,
    payload: web::Json<TodoCreateUpdate>,
) -> impl Responder {
    let mut guard = data.todo_list.lock().unwrap();
    let todo_list = &mut *guard;
    let payload = payload.into_inner();
    todo_list.insert(payload.title.unwrap(), payload.content.unwrap(), false);
    let str = serde_json::to_string(&todo_list).unwrap();
    HttpResponse::Ok().body(str)
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

pub(crate) async fn update(
    data: web::Data<ApplicationState>,
    path: web::Path<(usize,)>,
    payload: web::Json<TodoCreateUpdate>,
) -> impl Responder {
    let mut guard = data.todo_list.lock().unwrap();
    let todo_list = &mut *guard;
    let payload = payload.into_inner();

    todo_list.items.entry(path.0).and_modify(|v| {
        v.update(payload.title, payload.content, payload.checked);
    });
    HttpResponse::Ok().body("{\"detail\": \"Success\"}")
}

pub(crate) async fn delete(
    data: web::Data<ApplicationState>,
    path: web::Path<(usize,)>,
) -> impl Responder {
    let mut guard = data.todo_list.lock().unwrap();
    let todo_list = &mut *guard;

    todo_list.items.remove(&path.0);
    HttpResponse::Ok().body("{\"detail\": \"Success\"}")
}
