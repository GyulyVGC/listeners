use crate::helpers::{save_chart_svg, save_mean_txt};
use criterion::{Criterion, criterion_group, criterion_main};
use helpers::spawn_sockets;
use std::hint::black_box;

#[path = "helpers.rs"]
mod helpers;

fn benchmark_get_all_100(c: &mut Criterion) {
    // spawn sockets
    let _sockets = spawn_sockets(100);
    let listeners = listeners::get_all().unwrap_or_default().len();
    println!("=== Benchmarking get_all with {listeners} listeners ===");

    // benchmark
    let id = "get_all_100";
    benchmark_get_all(c, id);
}

fn benchmark_get_all_1k(c: &mut Criterion) {
    // spawn sockets
    let _sockets = spawn_sockets(1_000);
    let listeners = listeners::get_all().unwrap_or_default().len();
    println!("=== Benchmarking get_all with {listeners} listeners ===");

    // benchmark
    let id = "get_all_1k";
    benchmark_get_all(c, id);
}

fn benchmark_get_all_10k(c: &mut Criterion) {
    // spawn sockets
    let _sockets = spawn_sockets(10_000);
    let listeners = listeners::get_all().unwrap_or_default().len();
    println!("=== Benchmarking get_all with {listeners} listeners ===");

    // benchmark
    let id = "get_all_10k";
    benchmark_get_all(c, id);
}

fn benchmark_get_all(c: &mut Criterion, id: &str) {
    c.bench_function(id, |b| b.iter(|| black_box(listeners::get_all())));

    // save files
    save_chart_svg(id);
    save_mean_txt(id);
}

criterion_group!(
    benches,
    benchmark_get_all_100,
    benchmark_get_all_1k,
    benchmark_get_all_10k
);
criterion_main!(benches);
