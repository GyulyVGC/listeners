use crate::helpers::{get_ports_protos, save_chart_svg, save_mean_txt};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use helpers::spawn_sockets;
use listeners::Protocol;
use rand::prelude::IndexedRandom;
use std::hint::black_box;

#[path = "helpers.rs"]
mod helpers;

fn benchmark_get_process_by_port_100(c: &mut Criterion) {
    // spawn sockets
    let sockets = spawn_sockets(100);
    let ports_protos = get_ports_protos(&sockets);
    println!(
        "=== Benchmarking get_process_by_port with {} sockets ===",
        ports_protos.len()
    );

    // benchmark
    let id = "get_process_by_port_100";
    benchmark_get_process_by_port(c, id, ports_protos);
}

fn benchmark_get_process_by_port_1k(c: &mut Criterion) {
    // spawn sockets
    let sockets = spawn_sockets(1_000);
    let ports_protos = get_ports_protos(&sockets);
    println!(
        "=== Benchmarking get_process_by_port with {} sockets ===",
        ports_protos.len()
    );

    // benchmark
    let id = "get_process_by_port_1k";
    benchmark_get_process_by_port(c, id, ports_protos);
}

fn benchmark_get_process_by_port_10k(c: &mut Criterion) {
    // spawn sockets
    let sockets = spawn_sockets(10_000);
    let ports_protos = get_ports_protos(&sockets);
    println!(
        "=== Benchmarking get_process_by_port with {} sockets ===",
        ports_protos.len()
    );

    // benchmark
    let id = "get_process_by_port_10k";
    benchmark_get_process_by_port(c, id, ports_protos);
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

criterion_group!(
    benches,
    benchmark_get_process_by_port_100,
    benchmark_get_process_by_port_1k,
    benchmark_get_process_by_port_10k
);
criterion_main!(benches);
