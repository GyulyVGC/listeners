fn main() {
    for listener in listeners::get_all().iter().flatten() {
        println!("{listener}");
    }
}
