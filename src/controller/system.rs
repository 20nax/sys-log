use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json;

use crate::controller::ServerStateMut;

#[get("/cpu")]
async fn cpu_usage(data: web::Data<ServerStateMut>) -> impl Responder {
    let load = data.state.lock().unwrap().cpu_service.get_cpu_load();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&load).unwrap())
}

#[get("/temp")]
async fn all_comp_temp(data: web::Data<ServerStateMut>) -> impl Responder {
    let temps = data.state.lock().unwrap().cpu_service.get_all_temp();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&temps).unwrap())
}

#[get("/temp/{comp_name}")]
async fn comp_temp(
    data: web::Data<ServerStateMut>,
    web::Path(comp_name): web::Path<String>,
) -> impl Responder {
    let temp = data
        .state
        .lock()
        .unwrap()
        .cpu_service
        .get_temp(comp_name.as_str());

    match temp {
        Ok(value) => HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&value).unwrap()),
        Err(value) => HttpResponse::NotFound()
            .content_type("application/json")
            .body(serde_json::to_string(&value).unwrap()),
    }
}
