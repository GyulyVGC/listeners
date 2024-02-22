use listeners::get_all_listeners;

fn main() {
    for listener in get_all_listeners() {
        println!("{listener}");
    }
}
