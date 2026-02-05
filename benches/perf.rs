use std::hint::black_box;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, UdpSocket};
use criterion::{criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
enum SocketType {
    TCP(TcpListener),
    UDP(UdpSocket),
}

fn spawn_sockets(n: usize) -> Vec<SocketType> {
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

fn benchmark_get_all_100(c: &mut Criterion) {
    let _sockets = spawn_sockets(100);
    let listeners = listeners::get_all().unwrap_or_default().len();
    println!("=== Benchmarking get_all with {} listeners ===", listeners);

    c.bench_function("get_all__100", |b| b.iter(|| black_box(listeners::get_all())));
}

fn benchmark_get_all_1k(c: &mut Criterion) {
    let _sockets = spawn_sockets(1_000);
    let listeners = listeners::get_all().unwrap_or_default().len();
    println!("=== Benchmarking get_all with {} listeners ===", listeners);

    c.bench_function("get_all__1k", |b| b.iter(|| black_box(listeners::get_all())));
}

fn benchmark_get_all_10k(c: &mut Criterion) {
    let _sockets = spawn_sockets(10_000);
    let listeners = listeners::get_all().unwrap_or_default().len();
    println!("=== Benchmarking get_all with {} listeners ===", listeners);

    c.bench_function("get_all__10k", |b| b.iter(|| black_box(listeners::get_all())));
}

criterion_group!(benches, benchmark_get_all_100, benchmark_get_all_1k, benchmark_get_all_10k);
criterion_main!(benches);