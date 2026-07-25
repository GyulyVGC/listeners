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
    // A plain `::1` bind (vflag = INI_IPV6 only) must decode the v6 slot exactly;
    // the other IPv6 tests only match by port, so this pins the decoded value.
    assert_eq!(listener.socket.ip(), IpAddr::V6(Ipv6Addr::LOCALHOST));
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

/// An IPv4-mapped bind must report the address that was BOUND, not the
/// IPv4-compatible address that shares its low 32 bits.
///
/// Before the `insi_vflag` fix, macOS reported `::ffff:127.0.0.1` as `::127.0.0.1`:
/// `get_local_addr` branched on `soi_family` alone, so an AF_INET6 socket always had
/// its 16-byte `ina_6` slot read — but for a mapped bind the kernel keeps the address
/// in the 4-byte `i46a_addr4` slot and leaves `ina_6` holding 12 zero pad bytes plus
/// the v4 address. Read as IPv6 that is a different, deprecated address.
///
/// Kernel evidence for the same socket (macOS 27.0): `getsockname` returns AF_INET6
/// `::ffff:127.0.0.1`, while `proc_pidfdinfo` reports `insi_vflag = 0x01`
/// (INI_IPV4 set, INI_IPV6 CLEAR) with v4 slot `7f000001` and v6 slot
/// `0000000000000000000000007f000001`.
#[test]
#[serial]
fn test_tcp_ipv4_mapped_address_is_not_ipv4_compatible() {
    let mapped = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 0x0001); // ::ffff:127.0.0.1
    let tcp = TcpListener::bind(SocketAddr::new(IpAddr::V6(mapped), 0)).unwrap();
    let udp = UdpSocket::bind(SocketAddr::new(IpAddr::V6(mapped), 0)).unwrap();

    let all = listeners::get_all().unwrap();

    // The IPv4-COMPATIBLE address `::127.0.0.1` shares the low 32 bits with the
    // mapped one and is what the bug produced, so name it explicitly.
    let compatible = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0x7f00, 0x0001));

    // TCP reads `insi_vflag` from `pri_tcp.tcpsi_ini`, UDP from `pri_in` — a different
    // union member — so run the shared decode through both protocols.
    let cases = [
        (tcp.local_addr().unwrap().port(), Protocol::TCP),
        (udp.local_addr().unwrap().port(), Protocol::UDP),
    ];
    for (port, protocol) in cases {
        let listener = all
            .iter()
            .find(|l| l.socket.port() == port && l.protocol == protocol)
            .unwrap();

        assert_ne!(
            listener.socket.ip(),
            compatible,
            "{protocol:?}: reported the deprecated IPv4-compatible address instead of the mapped one",
        );

        // Whichever representation the platform uses, it must be loopback-equivalent:
        // `::ffff:127.0.0.1` and `127.0.0.1` are the same address, and both are loopback.
        let canonical = match listener.socket.ip() {
            IpAddr::V4(v4) => IpAddr::V4(v4),
            IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or(IpAddr::V6(v6), IpAddr::V4),
        };
        assert_eq!(
            canonical,
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            "{protocol:?}: an ::ffff:127.0.0.1 bind must canonicalize to 127.0.0.1, got {:?}",
            listener.socket.ip(),
        );
    }
}

/// A DUAL-STACK `::` bind sets BOTH `INI_IPV4` and `INI_IPV6` (`vflag = 0x03`), so the
/// fix's slot precedence matters: v6 must win, or the wildcard would be reported as
/// `0.0.0.0` and the socket's IPv6 reach would be lost. Paired with the test above
/// because the two together pin the ORDER of the two flag checks, which a single test
/// cannot.
#[test]
#[serial]
fn test_tcp_dual_stack_wildcard_reports_ipv6_unspecified() {
    let socket = TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0)).unwrap();
    let port = socket.local_addr().unwrap().port();

    let all = listeners::get_all().unwrap();
    let listener = all
        .iter()
        .find(|l| l.socket.port() == port && l.protocol == Protocol::TCP)
        .unwrap();

    assert_eq!(
        listener.socket.ip(),
        IpAddr::V6(Ipv6Addr::UNSPECIFIED),
        "a `::` bind must report the IPv6 wildcard, not the v4 slot's 0.0.0.0",
    );
}
