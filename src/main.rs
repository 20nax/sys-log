use sysinfo::{System, SystemExt};

mod service;

fn main() {
    let system = System::new_all();
    let mut cpu_service = service::cpu::CpuService::new(system);
    let mut memory_service = service::ram::RamService::new(System::new_all());
    let mut disk_service = service::disk::DiskService::new(System::new_all());

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
    println!("memory used : {}", res / 1000);

    let name_list = disk_service.get_disk_name();
    let available_list = disk_service.get_disk_available_space();
    let total_space_list = disk_service.get_disk_total_space();
    let size = name_list.len();

    for x in 0..size {
        println!(
            "Disk: {:?}, available: {} GB, total: {} GB",
            name_list[x],
            available_list[x] / 1000000000,
            total_space_list[x] / 1000000000
        )
    }
}
