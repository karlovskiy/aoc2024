use aoc2024::bridge_repair;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/bridge_repair/input.txt");
    let mut group = c.benchmark_group("part_one_benchmark");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("Bridge Repair (Part 1)", |b| {
        b.iter(|| bridge_repair::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/bridge_repair/input.txt");
    let mut group = c.benchmark_group("part_two_benchmark");
    group.measurement_time(Duration::from_secs(45));
    group.bench_function("Bridge Repair (Part 2)", |b| {
        b.iter(|| bridge_repair::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
