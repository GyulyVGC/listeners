use http_test_server::TestServer;
use listeners::{Listener, Process, Protocol};
use serial_test::serial;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, UdpSocket};
use std::str::FromStr;

#[test]
#[serial]
fn test_consistency() {
    // starts test server
    let _test = TestServer::new().unwrap();

    // retrieve all listeners and check that the set is not empty
    let listeners = listeners::get_all().unwrap();
    assert!(!listeners.is_empty());

    // maybe there is no udp connection
    if let Some(l_udp) = listeners.iter().find(|l| l.protocol == Protocol::UDP) {
        println!("UDP: {l_udp}");

        let ports_by_name = listeners::get_ports_by_process_name(&l_udp.process.name).unwrap();
        assert!(ports_by_name.contains(&l_udp.socket.port()));

        let processes_by_port = listeners::get_processes_by_port(l_udp.socket.port()).unwrap();
        assert!(processes_by_port.contains(&l_udp.process));

        let ports_by_pid = listeners::get_ports_by_pid(l_udp.process.pid).unwrap();
        assert!(ports_by_pid.contains(&l_udp.socket.port()));
    };

    if let Some(l_tcp) = listeners.iter().find(|l| l.protocol == Protocol::TCP) {
        println!("TCP: {l_tcp}");

        let ports_by_name = listeners::get_ports_by_process_name(&l_tcp.process.name).unwrap();
        assert!(ports_by_name.contains(&l_tcp.socket.port()));

        let processes_by_port = listeners::get_processes_by_port(l_tcp.socket.port()).unwrap();
        assert!(processes_by_port.contains(&l_tcp.process));

        let ports_by_pid = listeners::get_ports_by_pid(l_tcp.process.pid).unwrap();
        assert!(ports_by_pid.contains(&l_tcp.socket.port()));
    };
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
            socket: SocketAddr::from_str(&format!("127.0.0.1:{http_server_port}")).unwrap(),
            protocol: Protocol::TCP
        }
    );
}

#[test]
#[serial]
fn test_dns() {
    let dns_port = 53;
    let all = listeners::get_all().unwrap();
    let found = all.iter().any(|l| {
        l.socket.port() == dns_port && l.protocol == Protocol::UDP || l.protocol == Protocol::TCP
    });
    assert!(found);
}

#[test]
#[serial]
fn test_udp() {
    let mut opened_ports: Vec<u16> = Vec::new();
    let mut sockets: Vec<UdpSocket> = Vec::new();

    let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut current_port = 1500;
    let num_sockets = 10;

    for _ in 0..num_sockets {
        let socket = UdpSocket::bind(SocketAddr::new(ip_addr, current_port)).unwrap();
        current_port = socket.local_addr().unwrap().port();
        opened_ports.push(current_port);
        sockets.push(socket);
        current_port += 1;
    }

    let all_listeners = listeners::get_all().unwrap();
    let all_found = opened_ports.iter().all(|p| {
        all_listeners
            .iter()
            .any(|l| l.socket.port() == *p && l.protocol == Protocol::UDP)
    });

    assert!(all_found);
}

#[test]
#[serial]
fn test_tcp() {
    let mut opened_ports: Vec<u16> = Vec::new();
    let mut sockets: Vec<TcpListener> = Vec::new();

    let ip_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut current_port = 4500;
    let num_sockets = 10;

    for _ in 0..num_sockets {
        let socket = TcpListener::bind(SocketAddr::new(ip_addr, current_port)).unwrap();
        current_port = socket.local_addr().unwrap().port();
        opened_ports.push(current_port);
        sockets.push(socket);
        current_port += 1;
    }

    let all_listeners = listeners::get_all().unwrap();
    let all_found = opened_ports.iter().all(|p| {
        all_listeners
            .iter()
            .any(|l| l.socket.port() == *p && l.protocol == Protocol::TCP)
    });

    assert!(all_found);
}

#[test]
#[serial]
fn test_tcp6() {
    let mut opened_ports: Vec<u16> = Vec::new();
    let mut sockets: Vec<TcpListener> = Vec::new();

    let ip_addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    let mut current_port = 5600;
    let num_sockets = 10;

    for _ in 0..num_sockets {
        let socket = TcpListener::bind(SocketAddr::new(ip_addr, current_port)).unwrap();
        current_port = socket.local_addr().unwrap().port();
        opened_ports.push(current_port);
        sockets.push(socket);
        current_port += 1;
    }

    let all_listeners = listeners::get_all().unwrap();
    let all_found = opened_ports.iter().all(|p| {
        all_listeners
            .iter()
            .any(|l| l.socket.port() == *p && l.protocol == Protocol::TCP)
    });

    assert!(all_found);
}

#[test]
#[serial]
fn test_udp6() {
    let mut opened_ports: Vec<u16> = Vec::new();
    let mut sockets: Vec<UdpSocket> = Vec::new();

    let ip_addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    let mut current_port = 5600;
    let num_sockets = 10;

    for _ in 0..num_sockets {
        let socket = UdpSocket::bind(SocketAddr::new(ip_addr, current_port)).unwrap();
        current_port = socket.local_addr().unwrap().port();
        opened_ports.push(current_port);
        sockets.push(socket);
        current_port += 1;
    }

    let all_listeners = listeners::get_all().unwrap();
    let all_found = opened_ports.iter().all(|p| {
        all_listeners
            .iter()
            .any(|l| l.socket.port() == *p && l.protocol == Protocol::UDP)
    });

    assert!(all_found);
}
