# bigint-benchmark-rs

Benchmarks for Rust big integer implementations.

Each benchmark is run 5 times, with the calculation repeated for at least 10 seconds each time.
The median run is used as the result.

## Results

| Library                                             | Version | Notes                                | e 100k | e 1m   |  e 10m   | fib 1m  | fib 10m | fib_hex 10m |
| --------------                                      | ------- | ------                               | ----:  | -----: | -------: | ------: | ------: | ----------: |
| [rug](https://crates.io/crates/rug)                 | 1.11.0  | Links to [GMP](https://gmplib.org/)  | 0.016  |  0.296 |    4.520 |  0.015  |   0.336 | 0.060     |
| [rust-gmp](https://crates.io/crates/rust-gmp)       | 0.5.0   | Links to [GMP](https://gmplib.org/)  | 0.017  |  0.304 |    4.585 |  0.017  |   0.336 | 0.060     |
| [ibig](https://crates.io/crates/ibig)               | 0.2.1   | Pure Rust                            | 0.031  |  0.973 |   30.749 |  0.037  |   1.252 | 0.279     |
| [ramp](https://crates.io/crates/ramp)               | 0.5.9   | Uses assembly (requires nightly)     | 0.135  | 12.487 | 1233.225 |  0.355  |  34.847 | 0.386     |
| [num-bigint](https://crates.io/crates/num-bigint)   | 0.4.0   | Pure Rust, no `unsafe`               | 0.325  | 31.006 | 3098.511 |  1.161  | 115.234 | 0.401     |

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
ibig
Attempt 0: 16 iterations 977 ms
Attempt 1: 16 iterations 973 ms
Attempt 2: 16 iterations 973 ms
Attempt 3: 16 iterations 973 ms
Attempt 4: 16 iterations 973 ms
num-bigint
Attempt 0: 1 iterations 30965 ms
Attempt 1: 1 iterations 30960 ms
Attempt 2: 1 iterations 31006 ms
Attempt 3: 1 iterations 31090 ms
Attempt 4: 1 iterations 31075 ms
ramp
Attempt 0: 1 iterations 12488 ms
Attempt 1: 1 iterations 12489 ms
Attempt 2: 1 iterations 12485 ms
Attempt 3: 1 iterations 12486 ms
Attempt 4: 1 iterations 12487 ms
rug
Attempt 0: 64 iterations 296 ms
Attempt 1: 64 iterations 297 ms
Attempt 2: 64 iterations 297 ms
Attempt 3: 64 iterations 296 ms
Attempt 4: 64 iterations 296 ms
rust-gmp
Attempt 0: 64 iterations 305 ms
Attempt 1: 64 iterations 304 ms
Attempt 2: 64 iterations 304 ms
Attempt 3: 64 iterations 305 ms
Attempt 4: 64 iterations 304 ms
Results
rug        296 ms
rust-gmp   304 ms
ibig       973 ms
ramp       12487 ms
num-bigint 31006 ms
```

## Tasks

| Task      | Description                   | Difficulty | Algorithm             | Operations |
| ----      | ---------                     | ---------- | ---------             | ---------- |
| `e`       | n digits of e                 | Hard       | Binary splitting      | addition, multiplication, division, exponentiation, base conversion |
| `fib`     | n-th Fibonnaci number         | Medium     | Matrix exponentiation | addition, multiplication, base conversion |
| `fib_hex` | n-th Fibonnaci number in hex  | Easy       | Matrix exponentiation | addition, multiplication |
