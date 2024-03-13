fn main() {
    let listeners = listeners::get_all().unwrap();
    for l in listeners {
        println!("{l}");
    }
}
