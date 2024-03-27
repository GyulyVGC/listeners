use std::env::args;

fn main() {
    let port = args()
        .nth(1)
        .expect("Expected CLI argument: port")
        .parse()
        .expect("Port must be an unsigned integer on at most 16 bits");

    // Retrieve PID and name of processes listening on a given port
    if let Ok(processes) = listeners::get_processes_by_port(port) {
        for p in processes {
            println!("{p}");
        }
    }
}
