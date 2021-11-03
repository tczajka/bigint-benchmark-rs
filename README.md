# bigint-benchmark-rs

Benchmarks for Rust big integer implementations.

Each benchmark is run 5 times, with the calculation repeated for at least 10 seconds each time.
The fastest attempt is used as the result.

## Libraries

| Library                                             | Version | Notes                                               |
| --------------                                      | ------- | ------                                              |
| [rug](https://crates.io/crates/rug)                 | 1.13.0  | Links to libc and [GMP](https://gmplib.org/)        |
| [rust-gmp](https://crates.io/crates/rust-gmp)       | 0.5.0   | Links to libc and [GMP](https://gmplib.org/)        |
| [ibig](https://crates.io/crates/ibig)               | 0.3.4   | Pure Rust, no_std                                   |
| [num-bigint](https://crates.io/crates/num-bigint)   | 0.4.3   | Pure Rust, no_std                                   |
| [ramp](https://crates.io/crates/ramp)               | 0.6.0   | Requires nightly Rust, uses x86_64 assembly         |

## Results

| Library                                             | e 100k | e 1m   |  e 10m   | fib 10m | fib 100m | fib_hex 100m |
| --------------                                      | ----:  | -----: | -------: | ------: | -------: | -----------: |
| [rug](https://crates.io/crates/rug)                 | 0.016  |  0.270 |    4.522 | 0.335   | 5.907    | 0.974        |
| [rust-gmp](https://crates.io/crates/rust-gmp)       | 0.017  |  0.304 |    4.613 | 0.335   | 5.912    | 0.956        |
| [ibig](https://crates.io/crates/ibig)               | 0.033  |  0.975 |   30.838 | 1.192   | 40.566   | 8.676        |
| [num-bigint](https://crates.io/crates/num-bigint)   | 0.083  |  6.760 |  655.281 | 10.073  | 945.635  | 8.952        |
| [ramp](https://crates.io/crates/ramp)               | 0.135  | 12.567 | 1243.958 | 35.315  | 3555.541 | 16.671       |

Times in seconds.

## Usage

Calculate 100 digits of `e` and compare answers:
```
$ bigint-benchmark --task e --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp -n 100 print                                             
answer = 2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427
ibig       agrees
num-bigint agrees
ramp       agrees
rug        agrees
rust-gmp   agrees
```

Calculate the 500-th Fibonacci number and compare answers:
```
$ bigint-benchmark --task fib --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp -n 500 print                                            
answer = 139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125
ibig       agrees
num-bigint agrees
ramp       agrees
rug        agrees
rust-gmp   agrees
```

Benchmark calculating a million digits of e:
```
$ bigint-benchmark --task e --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp -n 1000000 benchmark
```

## Tasks

| Task      | Description                   | Difficulty | Algorithm             | Operations |
| ----      | ---------                     | ---------- | ---------             | ---------- |
| `e`       | n digits of e                 | Hard       | Binary splitting      | addition, multiplication, division, exponentiation, base conversion |
| `fib`     | n-th Fibonnaci number         | Medium     | Matrix exponentiation | addition, multiplication, base conversion |
| `fib_hex` | n-th Fibonnaci number in hex  | Easy       | Matrix exponentiation | addition, multiplication |
