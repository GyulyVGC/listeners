use listeners::get_all_listeners;

fn main() {
    // read second command line argument
    let mut args = std::env::args().skip(1);
    let pid = args.next();

    println!("===== Listeners =====\n");
    for listener in get_all_listeners(pid) {
        println!("{listener}");
    }
}
