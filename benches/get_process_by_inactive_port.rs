use crate::helpers::{SystemLoad, cleanup_bench, prepare_bench, save_chart_svg, save_info_txt};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use listeners::Protocol;
use rand::prelude::{IndexedRandom, IteratorRandom};
use std::collections::HashSet;
use std::hint::black_box;

#[path = "helpers.rs"]
mod helpers;

fn benchmark_get_process_by_inactive_port_low(c: &mut Criterion) {
    benchmark_get_process_by_inactive_port(c, SystemLoad::Low);
}

fn benchmark_get_process_by_inactive_port_medium(c: &mut Criterion) {
    benchmark_get_process_by_inactive_port(c, SystemLoad::Medium);
}

fn benchmark_get_process_by_inactive_port_high(c: &mut Criterion) {
    benchmark_get_process_by_inactive_port(c, SystemLoad::High);
}

fn benchmark_get_process_by_inactive_port(c: &mut Criterion, system_load: SystemLoad) {
    let id = match system_load {
        SystemLoad::Low => "get_process_by_inactive_port_low",
        SystemLoad::Medium => "get_process_by_inactive_port_medium",
        SystemLoad::High => "get_process_by_inactive_port_high",
    };

    // prepare bench
    let (sockets, bench_info) = prepare_bench(system_load);

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
    while inactive_ports_protos.len() < system_load.num_sockets() {
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
    save_info_txt(id, &bench_info);

    // cleanup bench
    cleanup_bench(sockets);
}

criterion_group!(
    benches,
    benchmark_get_process_by_inactive_port_low,
    benchmark_get_process_by_inactive_port_medium,
    benchmark_get_process_by_inactive_port_high
);
criterion_main!(benches);
