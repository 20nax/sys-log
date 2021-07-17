use sysinfo::{ComponentExt, System, SystemExt};

fn main() {
    let system = System::new_all();
    for component in system.components() {
        println!("{}\t\t{}", component.label(), component.temperature());
    }
}
