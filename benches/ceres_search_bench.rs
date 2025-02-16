use aoc2024::ceres_search;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/ceres_search/input.txt");
    c.bench_function("Ceres Search (Part 1)", |b| {
        b.iter(|| ceres_search::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/ceres_search/input.txt");
    c.bench_function("Ceres Search (Part 2)", |b| {
        b.iter(|| ceres_search::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
