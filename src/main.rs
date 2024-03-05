fn main() {
    let listeners = listeners::get_all().unwrap();
    for listener in listeners {
        println!("{listener:?}");
    }
}

// fn main() {
//     let print_title = |title: &str| {
//         let n_repeat = 40 - title.len() / 2;
//         let repeat_str = "=".repeat(n_repeat);
//         println!("\n{repeat_str} {title} {repeat_str}");
//     };
//
//     print_title("get_all()");
//     for listener in listeners::get_all() {
//         println!("{listener}");
//     }
//
//     let ip = IpAddr::from_str(
//         &std::env::args()
//             .nth(1)
//             .expect("Expected IP address as argument to program"),
//     )
//     .expect("The provided IP address is not valid");
//
//     hi_cross();
//
//     print_title(&format!("get_for_nullnet({ip})"));
//     for pname in listeners::get_for_nullnet(ip) {
//         println!("{pname}");
//     }
// }
