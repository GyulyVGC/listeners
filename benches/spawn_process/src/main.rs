use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

/// A process processes opening `n` ports.
fn main() {
    // read the first CLI argument as the number of sockets to spawn
    let n_string = std::env::args().nth(1).unwrap();
    let n: usize = n_string.parse().unwrap();

    // spawn n sockets
    let _sockets = spawn_sockets(n);

    // keep the process alive to allow the benchmark to run
    loop {
        sleep(Duration::from_secs(100));
    }
}

#[allow(dead_code)]
enum SocketType {
    Tcp(TcpListener),
    Udp(UdpSocket),
}

fn spawn_sockets(n: usize) -> Vec<SocketType> {
    let mut sockets: Vec<SocketType> = Vec::new();
    let socket_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);

    for _ in 0..n / 4 {
        let socket = TcpListener::bind(socket_v4).unwrap();
        sockets.push(SocketType::Tcp(socket));
    }
    for _ in 0..n / 4 {
        let socket = TcpListener::bind(socket_v6).unwrap();
        sockets.push(SocketType::Tcp(socket));
    }
    for _ in 0..n / 4 {
        let socket = UdpSocket::bind(socket_v4).unwrap();
        sockets.push(SocketType::Udp(socket));
    }
    for _ in 0..n / 4 {
        let socket = UdpSocket::bind(socket_v6).unwrap();
        sockets.push(SocketType::Udp(socket));
    }

    sockets
}
