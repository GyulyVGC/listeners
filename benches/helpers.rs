use listeners::Protocol;
use serde_json::Value;
use std::collections::HashSet;
use std::env::consts::OS;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, UdpSocket};

#[allow(dead_code)]
pub enum SocketType {
    TCP(TcpListener),
    UDP(UdpSocket),
}

pub fn prepare_bench(size: usize) -> (Vec<SocketType>, String) {
    // spawn sockets
    let sockets = spawn_sockets(size);

    // get bench info
    let bench_info = get_bench_info();
    println!("{bench_info}");

    (sockets, bench_info)
}

// TODO: sockets should be associated with different PIDs
fn spawn_sockets(n: usize) -> Vec<SocketType> {
    let mut sockets: Vec<SocketType> = Vec::new();
    let socket_v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    let socket_v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);

    for _ in 0..n / 4 {
        let socket = TcpListener::bind(socket_v4).unwrap();
        sockets.push(SocketType::TCP(socket));
    }
    for _ in 0..n / 4 {
        let socket = TcpListener::bind(socket_v6).unwrap();
        sockets.push(SocketType::TCP(socket));
    }
    for _ in 0..n / 4 {
        let socket = UdpSocket::bind(socket_v4).unwrap();
        sockets.push(SocketType::UDP(socket));
    }
    for _ in 0..n / 4 {
        let socket = UdpSocket::bind(socket_v6).unwrap();
        sockets.push(SocketType::UDP(socket));
    }

    sockets
}

fn get_bench_info() -> String {
    let listeners = listeners::get_all().unwrap_or_default();

    let n_listeners = listeners.len();

    let sockets = listeners
        .iter()
        .map(|listener| (listener.socket, listener.protocol))
        .collect::<HashSet<_>>();
    let n_sockets = sockets.len();

    let n_tcpv4 = sockets
        .iter()
        .filter(|(socket, protocol)| Protocol::TCP.eq(protocol) && socket.ip().is_ipv4())
        .count();
    let n_tcpv6 = sockets
        .iter()
        .filter(|(socket, protocol)| Protocol::TCP.eq(protocol) && socket.ip().is_ipv6())
        .count();
    let n_udpv4 = sockets
        .iter()
        .filter(|(socket, protocol)| Protocol::UDP.eq(protocol) && socket.ip().is_ipv4())
        .count();
    let n_udpv6 = sockets
        .iter()
        .filter(|(socket, protocol)| Protocol::UDP.eq(protocol) && socket.ip().is_ipv6())
        .count();

    let processes = listeners
        .iter()
        .map(|listener| listener.process.pid)
        .collect::<HashSet<_>>();
    let n_processes = processes.len();

    format!(
        "====================\n\
    - Listeners: {n_listeners}\n\
    - Processes: {n_processes}\n\
    - Sockets: {n_sockets}\n\
    \t- TCP (IPv4): {n_tcpv4}\n\
    \t- TCP (IPv6): {n_tcpv6}\n\
    \t- UDP (IPv4): {n_udpv4}\n\
    \t- UDP (IPv6): {n_udpv6}\n\
    ===================="
    )
}

#[allow(dead_code)]
pub fn get_ports_protos(sockets: &[SocketType]) -> Vec<(u16, Protocol)> {
    sockets
        .iter()
        .filter_map(|socket| match socket {
            SocketType::TCP(tcp) => tcp
                .local_addr()
                .ok()
                .map(|addr| (addr.port(), Protocol::TCP)),
            SocketType::UDP(udp) => udp
                .local_addr()
                .ok()
                .map(|addr| (addr.port(), Protocol::UDP)),
        })
        .collect::<Vec<_>>()
}

pub fn save_chart_svg(benchmark_id: &str) {
    let mut svg = std::fs::read_to_string(format!(
        "target/criterion/{benchmark_id}/report/pdf_small.svg"
    ))
    .unwrap();
    let insert_pos = svg.find('\n').unwrap() + 1;
    svg.insert_str(
        insert_pos,
        "<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n",
    );
    let dest = format!("resources/benchmarks/{OS}_{benchmark_id}.svg");
    std::fs::write(&dest, &svg).unwrap();
}

pub fn save_info_txt(benchmark_id: &str, bench_info: &str) {
    let json = std::fs::read_to_string(format!(
        "target/criterion/{benchmark_id}/new/estimates.json"
    ))
    .unwrap();
    let json: Value = serde_json::from_str(&json).unwrap();
    let mean_ns = json["mean"]["point_estimate"].as_f64().unwrap();
    let mean_ms = (mean_ns / 1_000_000.0).round() as usize;
    let dest = format!("resources/benchmarks/{OS}_{benchmark_id}.txt");
    std::fs::write(&dest, &format!("{bench_info}\n\n{mean_ms} ms")).unwrap();
}
