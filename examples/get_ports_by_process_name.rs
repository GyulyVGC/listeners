use std::env::args;

fn main() {
    let process_name = args().nth(1).expect("Expected CLI argument: process name");

    // Retrieve ports listened to by a process given its name
    if let Ok(ports) = listeners::get_ports_by_process_name(&process_name) {
        for p in ports {
            println!("{p}");
        }
    }
}
