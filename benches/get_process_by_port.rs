use crate::helpers::{get_ports_protos, save_chart_svg, save_mean_txt};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use helpers::spawn_sockets;
use listeners::Protocol;
use rand::prelude::{IndexedRandom, IteratorRandom};
use std::collections::HashSet;
use std::hint::black_box;

#[path = "helpers.rs"]
mod helpers;

fn benchmark_get_process_by_port_100(c: &mut Criterion) {
    let size = 100;

    // spawn sockets
    let sockets = spawn_sockets(size);
    let ports_protos = get_ports_protos(&sockets);
    println!(
        "=== Benchmarking get_process_by_port with {} sockets ===",
        ports_protos.len()
    );

    // benchmark with active ports
    let id = "get_process_by_port_100";
    benchmark_get_process_by_port(c, id, ports_protos);

    // benchmark with inactive ports
    let id = "get_process_by_inactive_port_100";
    benchmark_get_process_by_inactive_port(c, id, size);
}

fn benchmark_get_process_by_port_1k(c: &mut Criterion) {
    let size = 1_000;

    // spawn sockets
    let sockets = spawn_sockets(size);
    let ports_protos = get_ports_protos(&sockets);
    println!(
        "=== Benchmarking get_process_by_port with {} sockets ===",
        ports_protos.len()
    );

    // benchmark with active ports
    let id = "get_process_by_port_1k";
    benchmark_get_process_by_port(c, id, ports_protos);

    // benchmark with inactive ports
    let id = "get_process_by_inactive_port_1k";
    benchmark_get_process_by_inactive_port(c, id, size);
}

fn benchmark_get_process_by_port_10k(c: &mut Criterion) {
    let size = 10_000;

    // spawn sockets
    let sockets = spawn_sockets(size);
    let ports_protos = get_ports_protos(&sockets);
    println!(
        "=== Benchmarking get_process_by_port with {} sockets ===",
        ports_protos.len()
    );

    // benchmark with active ports
    let id = "get_process_by_port_10k";
    benchmark_get_process_by_port(c, id, ports_protos);

    // benchmark with inactive ports
    let id = "get_process_by_inactive_port_10k";
    benchmark_get_process_by_inactive_port(c, id, size);
}

fn benchmark_get_process_by_port(c: &mut Criterion, id: &str, ports_protos: Vec<(u16, Protocol)>) {
    let mut rng = rand::rng();
    c.bench_function(id, |b| {
        b.iter_batched(
            || *ports_protos.choose(&mut rng).unwrap(),
            |(port, protocol)| {
                black_box(
                    listeners::get_process_by_port(black_box(port), black_box(protocol)).unwrap(),
                )
            },
            BatchSize::SmallInput,
        )
    });

    // save files
    save_chart_svg(id);
    save_mean_txt(id);
}

fn benchmark_get_process_by_inactive_port(c: &mut Criterion, id: &str, size: usize) {
    let active_ports_protos: HashSet<(u16, Protocol)> = listeners::get_all()
        .unwrap()
        .into_iter()
        .map(|listener| (listener.socket.port(), listener.protocol))
        .collect();

    let mut rng = rand::rng();

    // use random ports/protocols that aren't in the list to test the "not found" case
    let all_ports: Vec<u16> = (1..u16::MAX).collect();
    let all_protocols = vec![Protocol::TCP, Protocol::UDP];
    let mut inactive_ports_protos = HashSet::new();
    while inactive_ports_protos.len() < size {
        let port = *all_ports.choose(&mut rng).unwrap();
        let protocol = *all_protocols.choose(&mut rng).unwrap();
        if !active_ports_protos.contains(&(port, protocol)) {
            inactive_ports_protos.insert((port, protocol));
        }
    }

    c.bench_function(id, |b| {
        b.iter_batched(
            || *inactive_ports_protos.iter().choose(&mut rng).unwrap(),
            |(port, protocol)| {
                black_box(
                    listeners::get_process_by_port(black_box(port), black_box(protocol))
                        .unwrap_err(),
                )
            },
            BatchSize::SmallInput,
        )
    });

    // save files
    save_chart_svg(id);
    save_mean_txt(id);
}

criterion_group!(
    benches,
    benchmark_get_process_by_port_100,
    benchmark_get_process_by_port_1k,
    benchmark_get_process_by_port_10k
);
criterion_main!(benches);
