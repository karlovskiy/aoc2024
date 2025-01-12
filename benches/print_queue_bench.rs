use aoc2024::print_queue;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/print_queue/input.txt");
    c.bench_function("Print Queue (Part 1)", |b| {
        b.iter(|| print_queue::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/print_queue/input.txt");
    c.bench_function("Print Queue (Part 2)", |b| {
        b.iter(|| print_queue::part_two(data))
    });
}

criterion_group! {
    name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(15));
  targets = part_one_benchmark, part_two_benchmark
}
criterion_main!(benches);
