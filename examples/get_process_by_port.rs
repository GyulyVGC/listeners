use std::env::args;

fn main() {
    let port = args()
        .nth(1)
        .expect("Expected CLI argument: port")
        .parse()
        .expect("Port must be an unsigned integer on at most 16 bits");
    let protocol_str = args()
        .next()
        .expect("Expected CLI argument: protocol (TCP or UDP)");

    let protocol = match protocol_str.to_uppercase().as_str() {
        "TCP" => listeners::Protocol::TCP,
        "UDP" => listeners::Protocol::UDP,
        _ => panic!("Protocol must be either TCP or UDP"),
    };

    // Retrieve PID and name of the process listening on a given port
    if let Ok(p) = listeners::get_process_by_port(port, protocol) {
        println!("{p}");
    }
}
