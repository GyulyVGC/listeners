use crate::helpers::{
    SystemLoad, cleanup_bench, get_ports_protos, prepare_bench, save_chart_svg, save_info_txt,
};
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
    let id = match system_load {
        SystemLoad::Low => "get_process_by_active_port_low",
        SystemLoad::Medium => "get_process_by_active_port_medium",
        SystemLoad::High => "get_process_by_active_port_high",
    };

    // prepare bench
    let (sockets, bench_info) = prepare_bench(system_load);

    let ports_protos = get_ports_protos(&sockets);
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
    save_info_txt(id, &bench_info);

    // cleanup bench
    cleanup_bench(sockets);
}

criterion_group!(
    benches,
    benchmark_get_process_by_active_port_low,
    benchmark_get_process_by_active_port_medium,
    benchmark_get_process_by_active_port_high
);
criterion_main!(benches);
