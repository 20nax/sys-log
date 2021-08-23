use actix_web::App;
use actix_web::HttpServer;
use std::sync::Arc;
use std::sync::Mutex;
use sysinfo::{System, SystemExt};

mod controller;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(controller::ServerStateMut {
                state: Arc::new(Mutex::new(controller::ServerState {
                    cpu_service: service::cpu::CpuService::new(System::new_all()),
                    disk_service: service::disk::DiskService::new(System::new_all()),
                    memory_service: service::ram::RamService::new(System::new_all()),
                })),
            })
            .service(controller::system::cpu_usage)
            .service(controller::system::all_comp_temp)
            .service(controller::system::comp_temp)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
