use http_test_server::TestServer;
use listeners::{Listener, Process, Protocol, SocketState, get_process_by_port};
use rand::prelude::IteratorRandom;
use serial_test::serial;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, UdpSocket};
use std::str::FromStr;

#[cfg(not(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd")))]
#[test]
#[serial]
fn test_consistency() {
    // starts test server
    let _test = TestServer::new().unwrap();

    // retrieve all listeners and check that the set is not empty
    let mut listeners = listeners::get_all().unwrap();
    assert!(!listeners.is_empty());

    // only keep listeners running on a port != 0
    listeners.retain(|l| l.socket.port() != 0);
    assert!(!listeners.is_empty());

    for l in &listeners {
        let process_by_port = listeners::get_process_by_port(l.socket.port(), l.protocol).unwrap();
        assert_eq!(process_by_port, l.process);
    }
}

#[test]
#[serial]
fn test_inactive_ports() {
    // starts test server in case there are no open sockets
    let _test = TestServer::new().unwrap();

    // retrieve all listeners and get their ports
    let ports = listeners::get_all()
        .unwrap()
        .iter()
        .map(|l| l.socket.port())
        .collect::<HashSet<_>>();
    assert!(!ports.is_empty());

    let mut inactive_ports = (1..u16::MAX).collect::<Vec<_>>();
    inactive_ports.retain(|p| !ports.contains(p));
    assert!(!inactive_ports.is_empty());

    // choose 10 random inactive ports and check that get_process_by_port returns an error for them
    let mut rng = rand::rng();
    let random_inactive_ports = inactive_ports.iter().sample(&mut rng, 10);
    for p in random_inactive_ports {
        let process_by_port = listeners::get_process_by_port(*p, Protocol::TCP);
        assert!(process_by_port.is_err());
        let process_by_port = listeners::get_process_by_port(*p, Protocol::UDP);
        assert!(process_by_port.is_err());
    }

    // also check that port 0 is error
    let process_by_port = listeners::get_process_by_port(0, Protocol::TCP);
    assert!(process_by_port.is_err());
    let process_by_port = listeners::get_process_by_port(0, Protocol::UDP);
    assert!(process_by_port.is_err());
}

#[test]
#[serial]
fn test_http_server() {
    // starts test server
    let http_server = TestServer::new().unwrap();
    let http_server_port = http_server.port();

    // get the http server process by its port
    let http_server_process =
        listeners::get_process_by_port(http_server_port, Protocol::TCP).unwrap();

    let http_server_name = http_server_process.name.clone();
    let http_server_pid = http_server_process.pid;
    let http_server_path = http_server_process.path.clone();

    // assert that the http server process name and path are not empty
    assert!(!http_server_name.is_empty());
    #[cfg(not(target_os = "openbsd"))]
    assert!(!http_server_path.is_empty());

    // get all listeners
    // and check that the http server is in the list
    let listeners = listeners::get_all().unwrap();
    let http_server_listener = listeners
        .iter()
        .find(|l| http_server_process.eq(&l.process))
        .unwrap();
    assert_eq!(
        http_server_listener,
        &Listener {
            process: Process {
                pid: http_server_pid,
                name: http_server_name,
                path: http_server_path
            },
            socket: SocketAddr::from_str(&format!("127.0.0.1:{http_server_port}")).unwrap(),
            protocol: Protocol::TCP,
            state: SocketState::Listen,
        }
    );
}

#[test]
#[serial]
fn test_dns() {
    let dns_port = 53;
    let all = listeners::get_all().unwrap();
    let found = all.iter().any(|l| {
        // assert that the process name is not empty
        assert!(!l.process.name.is_empty());
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
        let l = all_listeners
            .iter()
            .find(|l| l.socket.port() == *p && l.protocol == Protocol::UDP)
            .unwrap();
        let process_by_port = get_process_by_port(l.socket.port(), Protocol::UDP).unwrap();
        // assert that the process name and path are not empty
        assert!(!l.process.name.is_empty());
        #[cfg(not(target_os = "openbsd"))]
        assert!(!l.process.path.is_empty());
        l.process == process_by_port
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
        let l = all_listeners
            .iter()
            .find(|l| l.socket.port() == *p && l.protocol == Protocol::TCP)
            .unwrap();
        let process_by_port = get_process_by_port(l.socket.port(), Protocol::TCP).unwrap();
        // assert that the process name and path are not empty
        assert!(!l.process.name.is_empty());
        #[cfg(not(target_os = "openbsd"))]
        assert!(!l.process.path.is_empty());
        l.process == process_by_port
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
        let l = all_listeners
            .iter()
            .find(|l| l.socket.port() == *p && l.protocol == Protocol::TCP)
            .unwrap();
        let process_by_port = get_process_by_port(l.socket.port(), Protocol::TCP).unwrap();
        // assert that the process name and path are not empty
        assert!(!l.process.name.is_empty());
        #[cfg(not(target_os = "openbsd"))]
        assert!(!l.process.path.is_empty());
        l.process == process_by_port
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
        let l = all_listeners
            .iter()
            .find(|l| l.socket.port() == *p && l.protocol == Protocol::UDP)
            .unwrap();
        let process_by_port = get_process_by_port(l.socket.port(), Protocol::UDP).unwrap();
        // assert that the process name and path are not empty
        assert!(!l.process.name.is_empty());
        #[cfg(not(target_os = "openbsd"))]
        assert!(!l.process.path.is_empty());
        l.process == process_by_port
    });

    assert!(all_found);
}

#[test]
#[serial]
fn test_tcp_listen_state() {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = TcpListener::bind(SocketAddr::new(ip, 0)).unwrap();
    let port = socket.local_addr().unwrap().port();

    let all = listeners::get_all().unwrap();
    let listener = all
        .iter()
        .find(|l| l.socket.port() == port && l.protocol == Protocol::TCP)
        .unwrap();
    assert_eq!(listener.state, SocketState::Listen);
}

#[test]
#[serial]
fn test_tcp_established_state() {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let server = TcpListener::bind(SocketAddr::new(ip, 0)).unwrap();
    let port = server.local_addr().unwrap().port();

    let _client = std::net::TcpStream::connect(server.local_addr().unwrap()).unwrap();
    let (_accepted, _) = server.accept().unwrap();

    let all = listeners::get_all().unwrap();
    let has_established = all
        .iter()
        .any(|l| l.socket.port() == port && l.state == SocketState::Established);
    assert!(
        has_established,
        "Expected an established TCP connection on port {port}"
    );
}

#[test]
#[serial]
fn test_tcp_listen_state_ipv6() {
    let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    let socket = TcpListener::bind(SocketAddr::new(ip, 0)).unwrap();
    let port = socket.local_addr().unwrap().port();

    let all = listeners::get_all().unwrap();
    let listener = all
        .iter()
        .find(|l| l.socket.port() == port && l.protocol == Protocol::TCP)
        .unwrap();
    assert_eq!(listener.state, SocketState::Listen);
}

#[test]
#[serial]
fn test_tcp_established_state_ipv6() {
    let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    let server = TcpListener::bind(SocketAddr::new(ip, 0)).unwrap();
    let port = server.local_addr().unwrap().port();

    let _client = std::net::TcpStream::connect(server.local_addr().unwrap()).unwrap();
    let (_accepted, _) = server.accept().unwrap();

    let all = listeners::get_all().unwrap();
    let has_established = all
        .iter()
        .any(|l| l.socket.port() == port && l.state == SocketState::Established);
    assert!(
        has_established,
        "Expected an established IPv6 TCP connection on port {port}"
    );
}

#[test]
#[serial]
fn test_tcp_close_wait_state() {
    use std::io::Read;

    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let server = TcpListener::bind(SocketAddr::new(ip, 0)).unwrap();
    let port = server.local_addr().unwrap().port();

    let client = std::net::TcpStream::connect(server.local_addr().unwrap()).unwrap();
    let (mut accepted, _) = server.accept().unwrap();

    // Drop client to send FIN; read until EOF to confirm the FIN has been received
    // before sampling state, ensuring the kernel has moved accepted into CloseWait.
    drop(client);
    let mut buf = [0u8; 1];
    let _ = accepted.read(&mut buf);

    let all = listeners::get_all().unwrap();
    let has_close_wait = all
        .iter()
        .any(|l| l.socket.port() == port && l.state == SocketState::CloseWait);
    assert!(
        has_close_wait,
        "Expected a CloseWait TCP connection on port {port}"
    );
}

#[test]
#[serial]
fn test_udp_state_is_unknown() {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = UdpSocket::bind(SocketAddr::new(ip, 0)).unwrap();
    let port = socket.local_addr().unwrap().port();

    let all = listeners::get_all().unwrap();
    let listener = all
        .iter()
        .find(|l| l.socket.port() == port && l.protocol == Protocol::UDP)
        .unwrap();
    assert_eq!(listener.state, SocketState::Unknown);
}
