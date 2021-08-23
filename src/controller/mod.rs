pub mod db;
pub mod system;

use crate::service::cpu::CpuService;
use crate::service::disk::DiskService;
use crate::service::ram::RamService;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ServerState {
    pub cpu_service: CpuService,
    pub disk_service: DiskService,
    pub memory_service: RamService,
}

pub struct ServerStateMut {
    pub state: Arc<Mutex<ServerState>>,
}
