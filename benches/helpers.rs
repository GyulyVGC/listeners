use listeners::Protocol;
use rand::prelude::IndexedRandom;
use serde_json::Value;
use std::collections::HashSet;
use std::env::consts::OS;
use std::fmt::Display;
use std::process::Child;

pub struct BenchInfo {
    n_listeners: usize,
    n_processes: usize,
    n_sockets: usize,
    n_tcpv4: usize,
    n_tcpv6: usize,
    n_udpv4: usize,
    n_udpv6: usize,
    #[allow(dead_code)]
    pub active_ports_protos: Vec<(u16, Protocol)>,
    #[allow(dead_code)]
    pub inactive_ports_protos: Vec<(u16, Protocol)>,
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

        // to test the "not found" case: use random ports/protocols that aren't in the list
        let mut rng = rand::rng();
        let all_ports: Vec<u16> = (1..u16::MAX).collect();
        let all_protocols = vec![Protocol::TCP, Protocol::UDP];
        let mut inactive_ports_protos = Vec::new();
        while inactive_ports_protos.len() < 1_000 {
            let port = *all_ports.choose(&mut rng).unwrap();
            let protocol = *all_protocols.choose(&mut rng).unwrap();
            if !sockets.iter().any(|(active_sock, active_proto)| {
                active_sock.port() == port && active_proto == &protocol
            }) {
                inactive_ports_protos.push((port, protocol));
            }
        }

        // to test the "found" case: only get ports and protocols for spawned processes
        // (to avoid using ports from processes that might stop running while benchmarking)
        let active_ports_protos = listeners
            .iter()
            .filter(|listener| {
                #[cfg(target_os = "windows")]
                let process_name = "spawn_process.exe";
                #[cfg(not(target_os = "windows"))]
                let process_name = "spawn_process";

                listener.process.name == process_name && listener.socket.port() != 0
            })
            .map(|listener| (listener.socket.port(), listener.protocol))
            .collect();

        Self {
            n_listeners,
            n_processes,
            n_sockets,
            n_tcpv4,
            n_tcpv6,
            n_udpv4,
            n_udpv6,
            active_ports_protos,
            inactive_ports_protos,
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
            ..
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
    fn num_sockets(&self) -> usize {
        match self {
            SystemLoad::Low => 100,
            SystemLoad::Medium => 1_000,
            #[cfg(not(target_os = "openbsd"))]
            SystemLoad::High => 10_000,
            #[cfg(target_os = "openbsd")]
            SystemLoad::High => 1_000,
        }
    }

    fn num_processes(&self) -> usize {
        match self {
            SystemLoad::Low => 10,
            SystemLoad::Medium => 100,
            #[cfg(not(target_os = "openbsd"))]
            SystemLoad::High => 1_000,
            #[cfg(target_os = "openbsd")]
            SystemLoad::High => 100,
        }
    }

    pub fn activate(self) -> (Vec<Child>, BenchInfo) {
        // spawn processes
        let childs = self.spawn_processes();

        // get bench info
        let bench_info = BenchInfo::get();
        println!("{bench_info}");

        (childs, bench_info)
    }

    /// Spawns `tot_processes` processes, each opening `tot_sockets`/`tot_processes` ports.
    fn spawn_processes(self) -> Vec<Child> {
        let tot_processes = self.num_processes();
        let tot_sockets = self.num_sockets();
        let n_each = tot_sockets / tot_processes;

        println!("Spawning {tot_processes} processes, each opening {n_each} sockets...");

        let mut processes: Vec<Child> = Vec::new();

        for _ in 0..tot_processes {
            let process = std::process::Command::new("target/release/spawn_process")
                .arg(n_each.to_string())
                .spawn()
                .unwrap();
            processes.push(process);
        }

        // wait for processes to spawn and open their ports
        std::thread::sleep(std::time::Duration::from_secs(5));

        processes
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

pub fn cleanup(childs: Vec<Child>) {
    for mut process in childs {
        let _ = process.kill();
        let _ = process.wait();
    }
}

pub fn save_chart_svg(benchmark_id: &str, bench_info: &BenchInfo) {
    let mut svg = std::fs::read_to_string(format!(
        "target/criterion/{benchmark_id}/report/pdf_small.svg"
    ))
    .unwrap();
    let open_sockets = bench_info.n_sockets;
    let processes = bench_info.n_processes;
    let insert_pos = svg.find('\n').unwrap() + 1;
    svg.insert_str(
        insert_pos,
        &format!("<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n<text x=\"255\" y=\"15\" font-weight=\"bold\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"9.67741935483871\" opacity=\"1\" fill=\"#000000\">Processes: {processes} — Open ports: {open_sockets}</text>\n"),
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
