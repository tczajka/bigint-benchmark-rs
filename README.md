# bigint-benchmark-rs

Benchmarks for Rust big integer implementations.

Each benchmark is run 5 times, with the calculation repeated for at least 10 seconds each time.
The fastest attempt is used as the result.

## Libraries

| Library                                               | Version | Notes                                                  |
| --------------                                        | ------- | ------                                                 |
| [rug](https://crates.io/crates/rug)                   | 1.16.0  | Links to libc and [GMP](https://gmplib.org/)           |
| [rust-gmp](https://crates.io/crates/rust-gmp)         | 0.5.0   | Links to libc and [GMP](https://gmplib.org/)           |
| [ibig](https://crates.io/crates/ibig)                 | 0.3.5   | Pure Rust, no_std                                      |
| [malachite-nz](https://crates.io/crates/malachite-nz) | 0.2.2   | Pure Rust, LGPL, derived from GMP and FLINT            |
| [num-bigint](https://crates.io/crates/num-bigint)     | 0.4.3   | Pure Rust, no_std                                      |
| [ramp](https://crates.io/crates/ramp)                 | 0.7.0   | Requires nightly Rust, uses x86_64 assembly            |

## Results

| Library                                               | e 100k | e 1m   |  e 10m   | fib 10m | fib 100m | fib_hex 100m |
| --------------                                        | ----:  | -----: | -------: | ------: | -------: | -----------: |
| [rug](https://crates.io/crates/rug)                   | 0.016  |  0.269 |    4.940 | 0.333   | 5.880    | 1.027        |
| [rust-gmp](https://crates.io/crates/rust-gmp)         | 0.017  |  0.275 |    4.988 | 0.334   | 5.879    | 1.026        |
| [ibig](https://crates.io/crates/ibig)                 | 0.031  |  0.978 |   31.606 | 1.252   | 40.075   | 8.485        |
| [malachite-nz](https://crates.io/crates/malachite-nz) | 0.063  |  1.356 |   28.604 | 1.832   | 43.695   | 11.737       |
| [num-bigint](https://crates.io/crates/num-bigint)     | 0.086  |  6.776 |  656.515 | 10.271  | 950.146  | 8.967        |
| [ramp](https://crates.io/crates/ramp)                 | 0.136  | 12.565 | 1265.168 | 35.378  | 3563.058 | 16.603       |

Times in seconds.

## Usage

Calculate 100 digits of `e` and compare answers:
```
$ bigint-benchmark --task e --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp --lib malachite -n 100 print                                             
answer = 2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427
ibig       agrees
num-bigint agrees
ramp       agrees
rug        agrees
rust-gmp   agrees
malachite  agrees
```

Calculate the 500-th Fibonacci number and compare answers:
```
$ bigint-benchmark --task fib --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp --lib malachite -n 500 print                                            
answer = 139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125
ibig       agrees
num-bigint agrees
ramp       agrees
rug        agrees
rust-gmp   agrees
malachite  agrees
```

Benchmark calculating a million digits of e:
```
$ bigint-benchmark --task e --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp --lib malachite -n 1000000 benchmark
```

## Tasks

| Task      | Description                   | Difficulty | Algorithm             | Operations |
| ----      | ---------                     | ---------- | ---------             | ---------- |
| `e`       | n digits of e                 | Hard       | Binary splitting      | addition, multiplication, division, exponentiation, base conversion |
| `fib`     | n-th Fibonnaci number         | Medium     | Matrix exponentiation | addition, multiplication, base conversion |
| `fib_hex` | n-th Fibonnaci number in hex  | Easy       | Matrix exponentiation | addition, multiplication |
