use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let s = System::new_all();

    for process in s.get_process_by_name("ac_client") {
        println!("{} {} ", process.pid(), process.name());
    }
}