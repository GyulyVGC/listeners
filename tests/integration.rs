use std::net::SocketAddr;
use std::str::FromStr;

use http_test_server::TestServer;
use serial_test::serial;

use listeners::{Listener, Process};

#[test]
#[serial]
fn test_consistency() {
    // starts test server
    let _test = TestServer::new().unwrap();

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
#[serial]
fn test_http_server() {
    // starts test server
    let http_server = TestServer::new().unwrap();
    let http_server_port = http_server.port();

    // get the http server process by its port
    let processes = listeners::get_processes_by_port(http_server_port).unwrap();
    assert_eq!(processes.len(), 1);
    let http_server_process = processes.into_iter().next().unwrap();

    let http_server_name = http_server_process.name.clone();
    let http_server_pid = http_server_process.pid;

    // get the http server port by its process name
    // and check that it is the same as the one of the http server
    let ports = listeners::get_ports_by_process_name(&http_server_name).unwrap();
    assert_eq!(ports.len(), 1);
    assert!(ports.contains(&http_server_port));

    // get the http server port by its process id
    // and check that it is the same as the one of the http server
    let ports = listeners::get_ports_by_pid(http_server_pid).unwrap();
    assert_eq!(ports.len(), 1);
    assert!(ports.contains(&http_server_port));

    // get all listeners
    // and check that the http server is in the list
    let listeners = listeners::get_all().unwrap();
    let http_server_listener = listeners
        .iter()
        .find(|l| l.process.pid == http_server_pid)
        .unwrap();
    println!("{http_server_listener}");
    assert_eq!(
        http_server_listener,
        &Listener {
            process: Process {
                pid: http_server_pid,
                name: http_server_name
            },
            socket: SocketAddr::from_str(&format!("127.0.0.1:{http_server_port}")).unwrap()
        }
    );
}
