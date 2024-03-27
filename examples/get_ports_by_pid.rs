use std::env::args;

fn main() {
    let pid = args()
        .nth(1)
        .expect("Expected CLI argument: PID")
        .parse()
        .expect("PID must be an unsigned integer on at most 32 bits");

    // Retrieve ports listened to by a process given its PID
    if let Ok(ports) = listeners::get_ports_by_pid(pid) {
        for p in ports {
            println!("{p}");
        }
    }
}
