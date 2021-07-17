use sysinfo::{ComponentExt, System, SystemExt};

pub struct CpuService {
    system: System,
}

impl CpuService {
    pub fn new(system: System) -> CpuService {
        let res = CpuService { system: system };

        return res;
    }

    pub fn get_temp(&self, label: &str) -> Result<f32, String> {
        for component in self.system.components() {
            if component.label() == label {
                return Ok(component.temperature());
            }
        }

        Err(format!("Could not find device {}.", label))
    }
}
