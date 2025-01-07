# aoc2024
https://adventofcode.com/2024

## Day 1: Historian Hysteria
```
$ cargo bench --bench historian_hysteria_bench
    Finished `bench` profile [optimized] target(s) in 0.08s
     Running benches/historian_hysteria_bench.rs (target/release/deps/historian_hysteria_bench-6901f069c51c5637)
Historian Hysteria (Part 1)/data
                        time:   [54.250 µs 54.395 µs 54.570 µs]
                        change: [-1.9147% -0.6086% +0.3781%] (p = 0.35 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

Historian Hysteria (Part 2)/data
                        time:   [69.343 µs 69.520 µs 69.722 µs]
                        change: [+0.1798% +1.2811% +2.1329%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
```

## Day 2: Red-Nosed Reports
```
$ cargo bench --bench red_nosed_reports_bench
   Compiling aoc2024 v0.1.0 (/home/ravenstar/sources/aoc2024)
    Finished `bench` profile [optimized] target(s) in 0.87s
     Running benches/red_nosed_reports_bench.rs (target/release/deps/red_nosed_reports_bench-a2a0ec5368d9478b)
Red-Nosed Reports (Part 1)/data
                        time:   [30.488 µs 30.563 µs 30.658 µs]
                        change: [-25.206% -24.050% -22.930%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  7 (7.00%) high severe

Red-Nosed Reports (Part 2)/data
                        time:   [112.43 µs 112.77 µs 113.22 µs]
                        change: [+0.4833% +1.0705% +1.6922%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

```

## Day 3: Mull It Over
```
$ cargo bench --bench mull_it_over_bench
   Compiling aoc2024 v0.1.0 (/home/ravenstar/sources/aoc2024)
    Finished `bench` profile [optimized] target(s) in 1.10s
     Running benches/mull_it_over_bench.rs (target/release/deps/mull_it_over_bench-662c7dd85370e340)
Mull It Over (Part 1)   time:   [41.099 µs 41.200 µs 41.309 µs]
                        change: [-1.1797% -0.6073% +0.0830%] (p = 0.05 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

Mull It Over (Part 2)   time:   [30.303 µs 30.519 µs 30.791 µs]
                        change: [+0.3254% +0.8536% +1.5136%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  7 (7.00%) high severe

```
