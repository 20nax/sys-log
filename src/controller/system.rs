use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json;

use crate::controller::ServerStateMut;

#[get("/cpu")]
async fn all_cpu(data: web::Data<ServerStateMut>) -> impl Responder {
    let temps = data.state.lock().unwrap().cpu_service.get_cpu_load();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&temps).unwrap())
}
