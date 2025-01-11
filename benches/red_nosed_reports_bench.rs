use aoc2024::red_nosed_reports;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/red_nosed_reports/input.txt");
    c.bench_function("Red-Nosed Reports (Part 1)", |b| {
        b.iter(|| red_nosed_reports::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/red_nosed_reports/input.txt");
    c.bench_function("Red-Nosed Reports (Part 2)", |b| {
        b.iter(|| red_nosed_reports::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
