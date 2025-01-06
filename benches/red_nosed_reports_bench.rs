use aoc2024::red_nosed_reports;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

pub fn part_one_benchmark(c: &mut Criterion) {
    let lines = load_file();
    c.bench_with_input(
        BenchmarkId::new("Red-Nosed Reports (Part 1)", "data"),
        &lines,
        |b, i| b.iter(|| red_nosed_reports::part_one(&i)),
    );
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let lines = load_file();
    c.bench_with_input(
        BenchmarkId::new("Red-Nosed Reports (Part 2)", "data"),
        &lines,
        |b, i| b.iter(|| red_nosed_reports::part_two(&i)),
    );
}

fn load_file() -> Vec<String> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("benches/red_nosed_reports_input.txt");
    let file = File::open(d).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    lines
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);