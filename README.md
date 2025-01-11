# aoc2024
https://adventofcode.com/2024

## Day 1: Historian Hysteria
```
$ cargo bench --bench historian_hysteria_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/historian_hysteria_bench.rs (target/release/deps/historian_hysteria_bench-3d573a32d9e04d2c)
Historian Hysteria (Part 1)
                        time:   [63.393 µs 63.544 µs 63.719 µs]
                        change: [-0.5228% +0.0284% +0.5060%] (p = 0.92 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Historian Hysteria (Part 2)
                        time:   [77.757 µs 78.178 µs 78.840 µs]
                        change: [-0.4356% +0.1058% +0.5975%] (p = 0.71 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

```

## Day 2: Red-Nosed Reports
```
$ cargo bench --bench red_nosed_reports_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/red_nosed_reports_bench.rs (target/release/deps/red_nosed_reports_bench-322df0a54b806ce0)
Red-Nosed Reports (Part 1)
                        time:   [41.327 µs 41.531 µs 41.779 µs]
                        change: [-0.8625% -0.2213% +0.3731%] (p = 0.50 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

Red-Nosed Reports (Part 2)
                        time:   [117.74 µs 118.34 µs 119.00 µs]
                        change: [-0.3733% +0.4531% +1.2184%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

```

## Day 3: Mull It Over
```
$ cargo bench --bench mull_it_over_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/mull_it_over_bench.rs (target/release/deps/mull_it_over_bench-77b9bfac7970056d)
Mull It Over (Part 1)   time:   [50.237 µs 50.405 µs 50.618 µs]
                        change: [-1.2956% -0.3624% +0.3895%] (p = 0.43 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Mull It Over (Part 2)   time:   [44.619 µs 44.839 µs 45.098 µs]
                        change: [-1.6146% -0.6907% +0.4314%] (p = 0.19 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  6 (6.00%) high mild
  9 (9.00%) high severe

```

## Day 4: Ceres Search

```
$ cargo bench --bench ceres_search_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/ceres_search_bench.rs (target/release/deps/ceres_search_bench-6e70a9a8becd9de3)
Day 4: Ceres Search (Part 1)
                        time:   [122.11 µs 122.62 µs 123.16 µs]
                        change: [-0.2330% +0.4632% +1.2121%] (p = 0.24 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

Day 4: Ceres Search (Part 2)
                        time:   [49.257 µs 49.519 µs 49.798 µs]
                        change: [-0.0509% +0.9077% +1.8917%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

```