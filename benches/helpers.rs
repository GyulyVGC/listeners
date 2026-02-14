use listeners::Protocol;
use serde_json::Value;
use std::collections::HashSet;
use std::env::consts::OS;
use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, UdpSocket};

#[allow(dead_code)]
pub enum SocketType {
    TCP(TcpListener),
    UDP(UdpSocket),
}

pub struct BenchInfo {
    n_listeners: usize,
    n_processes: usize,
    n_sockets: usize,
    n_tcpv4: usize,
    n_tcpv6: usize,
    n_udpv4: usize,
    n_udpv6: usize,
}

impl BenchInfo {
    fn get() -> Self {
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

        Self {
            n_listeners,
            n_processes,
            n_sockets,
            n_tcpv4,
            n_tcpv6,
            n_udpv4,
            n_udpv6,
        }
    }
}

impl Display for BenchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let BenchInfo {
            n_listeners,
            n_processes,
            n_sockets,
            n_tcpv4,
            n_tcpv6,
            n_udpv4,
            n_udpv6,
        } = self;
        write!(
            f,
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
}

#[derive(Copy, Clone)]
pub enum SystemLoad {
    Low,
    Medium,
    High,
}

impl SystemLoad {
    pub fn num_sockets(&self) -> usize {
        match self {
            SystemLoad::Low => 100,
            SystemLoad::Medium => 1_000,
            SystemLoad::High => 10_000,
        }
    }

    // fn num_processes(&self) -> usize {
    //     match self {
    //         SystemLoad::Low => 10,
    //         SystemLoad::Medium => 100,
    //         SystemLoad::High => 1_000,
    //     }
    // }

    pub fn activate(self) -> (Vec<SocketType>, BenchInfo) {
        // spawn sockets
        let sockets = spawn_sockets(self.num_sockets());

        // spawn processes
        // let childs = spawn_processes(system_load.num_processes());

        // get bench info
        let bench_info = BenchInfo::get();
        println!("{bench_info}");

        (sockets, bench_info)
    }
}

impl Display for SystemLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemLoad::Low => write!(f, "low"),
            SystemLoad::Medium => write!(f, "medium"),
            SystemLoad::High => write!(f, "high"),
        }
    }
}

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

// TODO: increase the number of PIDs
// TODO: sockets should be associated with different PIDs
// fn spawn_processes(n: usize) -> Vec<Child> {
//     let mut processes: Vec<Child> = Vec::new();
//
//     for _ in 0..n {
//         #[cfg(not(target_os = "windows"))]
//         let program = "sleep";
//         #[cfg(target_os = "windows")]
//         let program = "timeout";
//         let process = std::process::Command::new(program)
//             .arg("1000")
//             .spawn()
//             .unwrap();
//         processes.push(process);
//     }
//
//     processes
// }

pub fn cleanup(sockets: Vec<SocketType>) {
    drop(sockets);
    // for mut process in processes {
    //     let _ = process.kill();
    //     let _ = process.wait();
    // }
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

pub fn save_chart_svg(benchmark_id: &str, bench_info: &BenchInfo) {
    let mut svg = std::fs::read_to_string(format!(
        "target/criterion/{benchmark_id}/report/pdf_small.svg"
    ))
    .unwrap();
    let open_sockets = bench_info.n_sockets;
    let insert_pos = svg.find('\n').unwrap() + 1;
    svg.insert_str(
        insert_pos,
        &format!("<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n<text x=\"255\" y=\"15\" font-weight=\"bold\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"9.67741935483871\" opacity=\"1\" fill=\"#000000\">Open sockets: {open_sockets}</text>\n"),
    );
    let dest = format!("resources/benchmarks/{OS}_{benchmark_id}.svg");
    std::fs::write(&dest, &svg).unwrap();
}

pub fn save_info_txt(benchmark_id: &str, bench_info: &BenchInfo) {
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
