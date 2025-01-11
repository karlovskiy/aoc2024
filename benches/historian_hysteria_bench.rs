use aoc2024::historian_hysteria;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/historian_hysteria/input.txt");
    c.bench_function("Historian Hysteria (Part 1)", |b| {
        b.iter(|| historian_hysteria::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/historian_hysteria/input.txt");
    c.bench_function("Historian Hysteria (Part 2)", |b| {
        b.iter(|| historian_hysteria::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
