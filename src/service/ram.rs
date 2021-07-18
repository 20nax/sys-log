use sysinfo::{System, SystemExt};

pub struct RamService {
    system: System,
}

impl RamService {
    pub fn new(system: System) -> RamService {
        let res = RamService { system: system };
        return res;
    }

    pub fn get_used_memory(&mut self) -> u64 {
        return self.system.used_memory();
    }
    pub fn get_total_memory(&mut self) -> u64 {
        return self.system.total_memory();
    }
}
