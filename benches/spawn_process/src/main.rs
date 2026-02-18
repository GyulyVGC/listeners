use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

/// A process processes opening `n` ports.
fn main() {
    // read the first CLI argument as the number of sockets to spawn
    let n_string = std::env::args().nth(1).unwrap();

    // spawn n sockets
    let _sockets = spawn_sockets(&n_string);

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

fn spawn_sockets(n_string: &str) -> Vec<SocketType> {
    let n: f32 = n_string.parse().unwrap();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n_4_up = (n / 4.0).ceil() as usize;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n_4_down = (n / 4.0).floor() as usize;

    let mut sockets: Vec<SocketType> = Vec::new();
    let socket_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);

    for _ in 0..n_4_up {
        let socket = TcpListener::bind(socket_v4).unwrap();
        sockets.push(SocketType::Tcp(socket));
    }
    for _ in 0..n_4_down {
        let socket = TcpListener::bind(socket_v6).unwrap();
        sockets.push(SocketType::Tcp(socket));
    }
    for _ in 0..n_4_up {
        let socket = UdpSocket::bind(socket_v4).unwrap();
        sockets.push(SocketType::Udp(socket));
    }
    for _ in 0..n_4_down {
        let socket = UdpSocket::bind(socket_v6).unwrap();
        sockets.push(SocketType::Udp(socket));
    }

    sockets
}
