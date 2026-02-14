use crate::helpers::{prepare_bench, save_chart_svg, save_info_txt};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[path = "helpers.rs"]
mod helpers;

fn benchmark_get_all_100(c: &mut Criterion) {
    let size = 100;

    let (_sockets, bench_info) = prepare_bench(size);

    // benchmark
    let id = "get_all_100";
    benchmark_get_all(c, id, &bench_info);
}

fn benchmark_get_all_1k(c: &mut Criterion) {
    let size = 1_000;

    let (_sockets, bench_info) = prepare_bench(size);

    // benchmark
    let id = "get_all_1k";
    benchmark_get_all(c, id, &bench_info);
}

fn benchmark_get_all_10k(c: &mut Criterion) {
    let size = 10_000;

    let (_sockets, bench_info) = prepare_bench(size);

    // benchmark
    let id = "get_all_10k";
    benchmark_get_all(c, id, &bench_info);
}

fn benchmark_get_all(c: &mut Criterion, id: &str, bench_info: &str) {
    c.bench_function(id, |b| b.iter(|| black_box(listeners::get_all())));

    // save files
    save_chart_svg(id);
    save_info_txt(id, bench_info);
}

criterion_group!(
    benches,
    benchmark_get_all_100,
    benchmark_get_all_1k,
    benchmark_get_all_10k
);
criterion_main!(benches);
