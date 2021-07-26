use sysinfo::{DiskExt, System, SystemExt};

pub struct DiskService {
    system: System,
}

impl DiskService {
    pub fn new(system: System) -> DiskService {
        let res = DiskService { system: system };
        return res;
    }

    pub fn get_disk_name(&self) -> Vec<&std::ffi::OsStr> {
        let mut vec: Vec<&std::ffi::OsStr> = Vec::new();
        for disk in self.system.disks() {
            vec.push(disk.name());
        }
        return vec;
    }

    pub fn get_disk_total_space(&self) -> Vec<u64> {
        let mut vec: Vec<u64> = Vec::new();
        for disk in self.system.disks() {
            vec.push(disk.total_space());
        }
        return vec;
    }

    pub fn get_disk_available_space(&self) -> Vec<u64> {
        let mut vec: Vec<u64> = Vec::new();
        for disk in self.system.disks() {
            vec.push(disk.available_space());
        }
        return vec;
    }
}
