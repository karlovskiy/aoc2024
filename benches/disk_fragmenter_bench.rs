use aoc2024::disk_fragmenter;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/disk_fragmenter/input.txt");
    c.bench_function("Disk Fragmenter (Part 1)", |b| {
        b.iter(|| disk_fragmenter::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/disk_fragmenter/input.txt");
    c.bench_function("Disk Fragmenter (Part 2)", |b| {
        b.iter(|| disk_fragmenter::part_one(data))
    });
}

criterion_group! {
    name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(45));
  targets = part_one_benchmark, part_two_benchmark
}
criterion_main!(benches);
