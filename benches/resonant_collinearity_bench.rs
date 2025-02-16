use aoc2024::resonant_collinearity;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/resonant_collinearity/input.txt");
    c.bench_function("Resonant Collinearity (Part 1)", |b| {
        b.iter(|| resonant_collinearity::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/resonant_collinearity/input.txt");
    c.bench_function("Resonant Collinearity (Part 2)", |b| {
        b.iter(|| resonant_collinearity::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
