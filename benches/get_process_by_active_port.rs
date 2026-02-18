use crate::helpers::{SystemLoad, cleanup, save_chart_svg, save_info_txt};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::prelude::IndexedRandom;
use std::hint::black_box;

#[path = "helpers.rs"]
mod helpers;

fn benchmark_get_process_by_active_port_low(c: &mut Criterion) {
    benchmark_get_process_by_active_port(c, SystemLoad::Low);
}

fn benchmark_get_process_by_active_port_medium(c: &mut Criterion) {
    benchmark_get_process_by_active_port(c, SystemLoad::Medium);
}

fn benchmark_get_process_by_active_port_high(c: &mut Criterion) {
    benchmark_get_process_by_active_port(c, SystemLoad::High);
}

fn benchmark_get_process_by_active_port(c: &mut Criterion, system_load: SystemLoad) {
    let id = format!("get_process_by_active_port_{system_load}");

    // prepare bench
    let (sockets, bench_info) = system_load.activate();

    let ports_protos = &bench_info.active_ports_protos;
    let mut rng = rand::rng();
    c.bench_function(&id, |b| {
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
    save_chart_svg(&id, &bench_info);
    save_info_txt(&id, &bench_info);

    // cleanup bench
    cleanup(sockets);
}

criterion_group!(
    benches,
    benchmark_get_process_by_active_port_low,
    benchmark_get_process_by_active_port_medium,
    benchmark_get_process_by_active_port_high
);
criterion_main!(benches);
