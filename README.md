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