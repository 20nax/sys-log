use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};

pub struct CpuService {
    system: System,
}

impl CpuService {
    pub fn new(system: System) -> CpuService {
        let res = CpuService { system: system };

        return res;
    }

    pub fn get_temp(&mut self, label: &str) -> Result<f32, String> {
        self.system.refresh_components();
        for component in self.system.components() {
            if component.label() == label {
                return Ok(component.temperature());
            }
        }
        Err(format!("Could not find device {}.", label))
    }

    pub fn get_cpu_load(&mut self) -> Vec<f32> {
        let mut vec: Vec<f32> = Vec::new();
        self.system.refresh_cpu();
        for processor in self.system.processors() {
            vec.push(processor.cpu_usage());
        }
        return vec;
    }
}
