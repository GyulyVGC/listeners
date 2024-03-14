fn main() {
    if let Ok(listeners) = listeners::get_all() {
        for l in listeners {
            println!("{l}");
        }
    }
}
