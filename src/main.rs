use sysinfo::{System, SystemExt};

mod service;

fn main() {
    let system = System::new_all();
    let cpu_service = service::cpu::CpuService::new(system);

    match cpu_service.get_temp("Core 0") {
        Ok(temp) => println!("Core 0 temp: {}", temp),
        Err(err) => eprintln!("{}", err),
    };
}
