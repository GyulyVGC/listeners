fn main() {
    // Retrieve all listeners
    if let Ok(listeners) = listeners::get_all() {
        for l in listeners {
            println!("{l}");
        }
    }
}
