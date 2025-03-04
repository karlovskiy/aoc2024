use aoc2024::hoof_it;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/hoof_it/input.txt");
    c.bench_function("Hoof It (Part 1)", |b| {
        b.iter(|| hoof_it::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/hoof_it/input.txt");
    c.bench_function("Hoof It (Part 2)", |b| {
        b.iter(|| hoof_it::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);