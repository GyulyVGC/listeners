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

#[test]
#[cfg(target_os = "linux")]
fn test_linux() {
    // starts test server
    use http_test_server::TestServer;
    let test_server = TestServer::new().unwrap();
    let test_server_port = test_server.port();

    // get process by port
    let processes = listeners::get_processes_by_port(test_server_port).unwrap();
    assert_eq!(processes.len(), 1);

    // get port by process
    let pid = processes.into_iter().next().unwrap().pid;
    let ports = listeners::get_ports_by_pid(pid).unwrap();
    assert_eq!(ports.len(), 1);
    assert_eq!(ports.into_iter().next().unwrap(), test_server_port);
}
