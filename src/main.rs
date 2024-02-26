use listeners::get_all_listeners;

fn main() {
    let print_title = |title: &str| {
        let n_repeat = 40 - title.len() / 2;
        let repeat_str = "=".repeat(n_repeat);
        println!("\n{repeat_str} {title} {repeat_str}");
    };

    print_title("get_all_listeners()");
    for listener in get_all_listeners() {
        println!("{listener}");
    }
}
