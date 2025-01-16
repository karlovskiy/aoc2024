use aoc2024::guard_gallivant;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/guard_gallivant/input.txt");
    c.bench_function("Guard Gallivant (Part 1)", |b| {
        b.iter(|| guard_gallivant::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/guard_gallivant/input.txt");
    let mut group = c.benchmark_group("part_two_benchmark");
    group.measurement_time(Duration::from_secs(300));
    group.bench_function("Guard Gallivant (Part 2)", |b| {
        b.iter(|| guard_gallivant::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
