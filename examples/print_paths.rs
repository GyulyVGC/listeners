fn main() {
    // Retrieve all listeners and print their process paths
    if let Ok(listeners) = listeners::get_all() {
        for l in listeners {
            println!(
                "PID: {:<10} Process path: {}",
                l.process.pid, l.process.path
            );
        }
    }
}
