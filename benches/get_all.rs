use crate::helpers::{SystemLoad, cleanup_bench, prepare_bench, save_chart_svg, save_info_txt};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[path = "helpers.rs"]
mod helpers;

fn benchmark_get_all_low(c: &mut Criterion) {
    benchmark_get_all(c, SystemLoad::Low);
}

fn benchmark_get_all_medium(c: &mut Criterion) {
    benchmark_get_all(c, SystemLoad::Medium);
}

fn benchmark_get_all_high(c: &mut Criterion) {
    benchmark_get_all(c, SystemLoad::High);
}

fn benchmark_get_all(c: &mut Criterion, system_load: SystemLoad) {
    let id = match system_load {
        SystemLoad::Low => "get_all_low",
        SystemLoad::Medium => "get_all_medium",
        SystemLoad::High => "get_all_high",
    };

    // prepare bench
    let (sockets, bench_info) = prepare_bench(system_load);

    c.bench_function(id, |b| b.iter(|| black_box(listeners::get_all())));

    // save files
    save_chart_svg(id);
    save_info_txt(id, &bench_info);

    // cleanup bench
    cleanup_bench(sockets);
}

criterion_group!(
    benches,
    benchmark_get_all_low,
    benchmark_get_all_medium,
    benchmark_get_all_high
);
criterion_main!(benches);
