fn main() {
    let listeners = listeners::get_all().unwrap();
    for listener in listeners {
        println!("{listener}");
    }
}
