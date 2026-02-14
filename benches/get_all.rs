use crate::helpers::{SystemLoad, cleanup, save_chart_svg, save_info_txt};
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
    let id = format!("get_all_{system_load}");

    // prepare bench
    let (sockets, bench_info) = system_load.activate();

    c.bench_function(&id, |b| b.iter(|| black_box(listeners::get_all())));

    // save files
    save_chart_svg(&id, &bench_info);
    save_info_txt(&id, &bench_info);

    // cleanup bench
    cleanup(sockets);
}

criterion_group!(
    benches,
    benchmark_get_all_low,
    benchmark_get_all_medium,
    benchmark_get_all_high
);
criterion_main!(benches);
