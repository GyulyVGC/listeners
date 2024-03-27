use listeners;

#[test]
#[cfg(not(target_os = "windows"))]
fn test_get_all_is_not_empty() {
    println!("----- test_get_all_is_not_empty() -----");
    let listeners = listeners::get_all().unwrap();
    assert!(!listeners.is_empty());
    for l in listeners {
        println!("{l}");
    }
}
