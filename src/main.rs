use listeners::get_all_listeners;

fn main() {
    let listeners = get_all_listeners();

    println!("===== Listeners =====\n");
    for listener in listeners {
        println!("{listener}");
    }
}
