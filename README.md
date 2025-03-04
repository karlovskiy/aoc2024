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
                        time:   [123.44 µs 123.73 µs 124.04 µs]
                        change: [-0.4874% +0.1368% +0.7055%] (p = 0.66 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

Day 4: Ceres Search (Part 2)
                        time:   [41.755 µs 41.985 µs 42.213 µs]
                        change: [-0.9789% +0.0637% +0.9969%] (p = 0.90 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```

## Day 5: Print Queue
```
$ cargo bench --bench print_queue_bench
    Finished `bench` profile [optimized] target(s) in 0.92s
     Running benches/print_queue_bench.rs (target/release/deps/print_queue_bench-d41d96e0bd714bc5)
Print Queue (Part 1)    time:   [233.75 µs 235.74 µs 238.10 µs]
                        change: [+0.0048% +0.7279% +1.5264%] (p = 0.05 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe

Print Queue (Part 2)    time:   [3.6260 ms 3.6319 ms 3.6387 ms]
                        change: [-4.4110% -2.8852% -1.5670%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  3 (3.00%) high severe
```

## Day 6: Guard Gallivant
```
$ cargo bench --bench guard_gallivant_bench
    Finished `bench` profile [optimized] target(s) in 1.45s
     Running benches/guard_gallivant_bench.rs (target/release/deps/guard_gallivant_bench-d01d7bdcf149c3eb)
Guard Gallivant (Part 1)
                        time:   [128.47 µs 128.85 µs 129.34 µs]
                        change: [-2.6999% -1.9938% -1.3167%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

part_two_benchmark/Guard Gallivant (Part 2)
                        time:   [1.6155 s 1.6218 s 1.6323 s]
                        change: [-7.9423% -7.5821% -7.0223%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe
```

## Day 7: Bridge Repair
```
$ cargo bench --bench bridge_repair_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/bridge_repair_bench.rs (target/release/deps/bridge_repair_bench-5f654c3ae390cbea)
part_one_benchmark/Bridge Repair (Part 1)
                        time:   [993.59 µs 995.46 µs 997.84 µs]
                        change: [-0.3498% +0.0565% +0.4786%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

part_two_benchmark/Bridge Repair (Part 2)
                        time:   [384.54 ms 384.79 ms 385.08 ms]
                        change: [-0.1410% -0.0447% +0.0524%] (p = 0.38 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe
```

## Day 8: Resonant Collinearity
```
$ cargo bench --bench resonant_collinearity_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/resonant_collinearity_bench.rs (target/release/deps/resonant_collinearity_bench-235ad185b6e2d532)
Day 8: Resonant Collinearity (Part 1)
                        time:   [181.67 µs 183.32 µs 185.49 µs]
                        change: [-3.7732% -1.4292% +0.4060%] (p = 0.21 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

Day 8: Resonant Collinearity (Part 2)
                        time:   [379.12 µs 381.95 µs 385.12 µs]
                        change: [-0.9173% +0.3904% +1.7637%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
```

## Day 9: Disk Fragmenter
```
$ cargo bench --bench disk_fragmenter_bench
   Compiling aoc2024 v0.1.0 (/home/ravenstar/sources/aoc2024)
    Finished `bench` profile [optimized] target(s) in 0.91s
     Running benches/disk_fragmenter_bench.rs (target/release/deps/disk_fragmenter_bench-6f01ca9093573634)
Disk Fragmenter (Part 1)
                        time:   [403.74 ms 406.27 ms 410.04 ms]
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) high mild
  14 (14.00%) high severe

Disk Fragmenter (Part 2)
                        time:   [403.24 ms 403.63 ms 404.09 ms]
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high severe
```
## Day 10: Hoof It
```
$ cargo bench --bench hoof_it_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/hoof_it_bench.rs (target/release/deps/hoof_it_bench-56e858f88ba7e065)
Hoof It (Part 1)        time:   [29.624 µs 29.768 µs 29.945 µs]
                        change: [-0.6254% +0.2448% +1.1460%] (p = 0.61 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

Hoof It (Part 2)        time:   [21.995 µs 22.114 µs 22.242 µs]
                        change: [+0.5151% +1.5257% +2.6821%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

```
