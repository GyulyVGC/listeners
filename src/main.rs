use listeners::get_all_listeners;

fn main() {
    let mut args = std::env::args().skip(1);
    let pid = args.next();

    let listeners = get_all_listeners(pid);

    println!("===== Listeners =====\n");
    for listener in listeners {
        println!("{listener}");
    }
}
