use listeners::Protocol;
use serde_json::Value;
use std::env::consts::OS;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, UdpSocket};

#[allow(dead_code)]
pub enum SocketType {
    TCP(TcpListener),
    UDP(UdpSocket),
}

pub fn spawn_sockets(n: usize) -> Vec<SocketType> {
    let mut sockets: Vec<SocketType> = Vec::new();
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    for _ in 0..n / 2 {
        let socket = TcpListener::bind(socket).unwrap();
        sockets.push(SocketType::TCP(socket));
    }
    for _ in 0..n / 2 {
        let socket = UdpSocket::bind(socket).unwrap();
        sockets.push(SocketType::UDP(socket));
    }
    sockets
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

pub fn save_mean_txt(benchmark_id: &str) {
    let json = std::fs::read_to_string(format!(
        "target/criterion/{benchmark_id}/new/estimates.json"
    ))
    .unwrap();
    let json: Value = serde_json::from_str(&json).unwrap();
    let mean_ns = json["mean"]["point_estimate"].as_f64().unwrap();
    let mean_ms = (mean_ns / 1_000_000.0).round() as usize;
    let dest = format!("resources/benchmarks/{OS}_{benchmark_id}.txt");
    std::fs::write(&dest, &format!("{mean_ms} ms")).unwrap();
}
