use sysinfo::{System, SystemExt};

mod service;

fn main() {
    let system = System::new_all();
    let mut cpu_service = service::cpu::CpuService::new(system);
    let mut memory_service = service::ram::RamService::new(System::new_all());

    match cpu_service.get_temp("Core 0") {
        Ok(temp) => println!("Core 0 temp: {}", temp),
        Err(err) => eprintln!("{}", err),
    };

    let res = cpu_service.get_cpu_load();
    for elemen in res {
        println!("cpu load : {}", elemen);
    }

    let res = memory_service.get_total_memory();
    // optionnal division to increase readability
    println!("memory total : {}", res / 1000);

    let res = memory_service.get_used_memory();
    // optionnal division to increase readability
    println!("memory available : {}", res / 1000);
}
