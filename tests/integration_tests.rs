use listeners;

#[test]
#[cfg(not(target_os = "linux"))]
fn test_consistency() {
    // retrieve all listeners and check that the set is not empty
    let listeners = listeners::get_all().unwrap();
    assert!(!listeners.is_empty());

    // check that the listeners retrieved by the different APIs are consistent
    for l in listeners {
        println!("{l}");

        let ports_by_pid = listeners::get_ports_by_pid(l.process.pid).unwrap();
        assert!(ports_by_pid.contains(&l.socket.port()));

        let ports_by_name = listeners::get_ports_by_process_name(&l.process.name).unwrap();
        assert!(ports_by_name.contains(&l.socket.port()));

        let processes_by_port = listeners::get_processes_by_port(l.socket.port()).unwrap();
        assert!(processes_by_port.contains(&l.process));
    }
}

#[test]
#[cfg(target_os = "windows")]
fn test_windows() {
    // check that the "System" process is listening on Windows
    let ports = listeners::get_ports_by_process_name("System").unwrap();
    assert!(!ports.is_empty());
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos() {
    // check that the "launchd" process is listening on macOS
    let ports = listeners::get_ports_by_process_name("launchd").unwrap();
    assert!(!ports.is_empty());
}
