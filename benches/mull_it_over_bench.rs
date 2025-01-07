use aoc2024::mull_it_over;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/mull_it_over/input.txt");
    c.bench_function("Mull It Over (Part 1)", |b| {
        b.iter(|| mull_it_over::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/mull_it_over/input.txt");
    c.bench_function("Mull It Over (Part 2)", |b| {
        b.iter(|| mull_it_over::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
