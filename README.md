# bigint-benchmark-rs

Benchmarks for Rust big integer implementations.

Each benchmark is run 5 times, with the calculation repeated for at least 10 seconds each time.
The fastest attempt is used as the result.

## Libraries

| Library                                               | Version | License  | Notes                                                  |
| --------------                                        | ------- | -------- | ------                                                 |
| [rug](https://crates.io/crates/rug)                   | 1.17.0  | LGPL 3.0 | Links to [GMP](https://gmplib.org/)                    |
| [malachite](https://crates.io/crates/malachite)       | 0.3.0   | LGPL 3.0 | Pure Rust, derived from [GMP](https://gmplib.org) and [FLINT](https://www.flintlib.org/) |
| [ibig](https://crates.io/crates/ibig)                 | 0.3.5   | MIT or Apache 2.0 | Pure Rust, no_std                            |
| [num-bigint](https://crates.io/crates/num-bigint)     | 0.4.3   | MIT or Apache 2.0 | Pure Rust, no_std                            |
| [dashu](https://crates.io/crates/dashu)               | 0.4.0   | MIT or Apache 2.0 | Pure Rust, no_std                            |

## Results

| Library                                               | e 100k | e 1m   |  e 10m   | fib 10m | fib 100m | fib_hex 100m |
| --------------                                        | ----:  | -----: | -------: | ------: | -------: | -----------: |
| [rug](https://crates.io/crates/rug)                   | 0.014  |  0.268 |    4.493 | 0.304   | 5.367    | 0.949        |
| [malachite](https://crates.io/crates/malachite)       | 0.043  |  0.929 |   15.798 | 1.197   | 21.473   | 3.270        |
| [ibig](https://crates.io/crates/ibig)                 | 0.031  |  0.978 |   31.606 | 1.252   | 40.075   | 8.485        |
| [num-bigint](https://crates.io/crates/num-bigint)     | 0.086  |  6.776 |  656.515 | 10.271  | 950.146  | 8.967        |
| [dashu](https://crates.io/crates/dashu)               | x.xxx  |  x.xxx |    x.xxx |  x.xxx  |   x.xxx  | x.xxx        |

Times in seconds.

## Usage

Calculate 100 digits of `e` and compare answers:
```
$ bigint-benchmark --task e --lib ibig --lib num-bigint --lib rug --lib malachite --lib dashu -n 100 print                                             
answer = 2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427
ibig       agrees
num-bigint agrees
rug        agrees
malachite  agrees
dashu      agrees
```

Calculate the 500-th Fibonacci number and compare answers:
```
$ bigint-benchmark --task fib --lib ibig --lib num-bigint --lib rug --lib malachite --lib dashu -n 500 print                                            
answer = 139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125
ibig       agrees
num-bigint agrees
rug        agrees
malachite  agrees
dashu      agrees
```

Benchmark calculating a million digits of e:
```
$ bigint-benchmark --task e --lib ibig --lib num-bigint --lib rug --lib malachite --lib dashu -n 1000000 benchmark
```

## Tasks

| Task      | Description                   | Difficulty | Algorithm             | Operations |
| ----      | ---------                     | ---------- | ---------             | ---------- |
| `e`       | n digits of e                 | Hard       | Binary splitting      | addition, multiplication, division, exponentiation, base conversion |
| `fib`     | n-th Fibonnaci number         | Medium     | Matrix exponentiation | addition, multiplication, base conversion |
| `fib_hex` | n-th Fibonnaci number in hex  | Easy       | Matrix exponentiation | addition, multiplication |
