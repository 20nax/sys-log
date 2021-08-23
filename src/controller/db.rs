use crate::service;
use std::time;
use sysinfo::{System, SystemExt};

pub fn cpu_write(mut con: postgres::Client) -> Result<postgres::Client, postgres::Error> {
    let mut cpu_service: service::cpu::CpuService =
        service::cpu::CpuService::new(System::new_all());

    let cpu_load: Vec<i32> = cpu_service
        .get_cpu_load()
        .into_iter()
        .map(|e| e as i32)
        .collect();

    if cpu_load.len() < 3 {
        panic!("Hardcoding values is bad");
    }

    let cpu_temp: Vec<(String, i32)> = cpu_service
        .get_all_temp()
        .into_iter()
        .map(|e| (e.0, e.1 as i32))
        .collect();

    if cpu_temp.len() < 3 {
        panic!("Hardcoding values is still bad");
    }

    let timestamp = time::SystemTime::now();
    con.execute(
    "INSERT INTO cpu (date, core0_u, core1_u, core2_u, core3_u, core0_t, core1_t, core2_t, core3_t) \
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    &[&timestamp, &cpu_load[0] , &cpu_load[1],&cpu_load[2],&cpu_load[3],&cpu_temp[0].1, &cpu_temp[1].1,&cpu_temp[2].1,&cpu_temp[3].1])?;
    return Ok(con);
}
