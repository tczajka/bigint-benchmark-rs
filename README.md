# bigint-benchmark-rs

Benchmarks for Rust big integer implementations.

Each benchmark is run 5 times, with the calculation repeated for at least 10 seconds each time.
The fastest attempt is used as the result.

## Libraries

| Library                                           | Version | License           | Notes                                                                                    |
|---------------------------------------------------|---------|-------------------|------------------------------------------------------------------------------------------|
| [rug](https://crates.io/crates/rug)               | 1.24.1  | LGPL 3.0          | Links to [GMP](https://gmplib.org/)                                                      |
| [malachite](https://crates.io/crates/malachite)   | 0.4.14  | LGPL 3.0          | Pure Rust, derived from [GMP](https://gmplib.org) and [FLINT](https://www.flintlib.org/) |
| [dashu](https://crates.io/crates/dashu)           | 0.4.2   | MIT or Apache 2.0 | Fork of [ibig](https://crates.io/crates/ibig). Pure Rust, no_std                         |
| [ibig](https://crates.io/crates/ibig)             | 0.3.6   | MIT or Apache 2.0 | Pure Rust, no_std                                                                        |
| [num-bigint](https://crates.io/crates/num-bigint) | 0.4.6   | MIT or Apache 2.0 | Pure Rust, no_std                                                                        |


## Results

| Library                                           | e 100k |  e 1m |   e 10m | fib 10m | fib 100m | fib_hex 100m |
|---------------------------------------------------|-------:|------:|--------:|--------:|---------:|-------------:|
| [rug](https://crates.io/crates/rug)               |  0.009 | 0.184 |   2.788 |   0.171 |    2.937 |        0.575 |
| [malachite](https://crates.io/crates/malachite)   |  0.012 | 0.240 |   3.689 |   0.306 |    5.192 |        0.933 |
| [dashu](https://crates.io/crates/dashu)           |  0.019 | 0.756 |  19.943 |   0.773 |   26.224 |        6.718 |
| [ibig](https://crates.io/crates/ibig)             |  0.020 | 0.649 |  20.673 |   0.793 |   26.705 |        6.725 |
| [num-bigint](https://crates.io/crates/num-bigint) |  0.058 | 5.037 | 482.383 |   7.007 |  671.809 |        6.312 |

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

| Task      | Description                  | Difficulty | Algorithm             | Operations                                                          |
|-----------|------------------------------|------------|-----------------------|---------------------------------------------------------------------|
| `e`       | n digits of e                | Hard       | Binary splitting      | addition, multiplication, division, exponentiation, base conversion |
| `fib`     | n-th Fibonnaci number        | Medium     | Matrix exponentiation | addition, multiplication, base conversion                           |
| `fib_hex` | n-th Fibonnaci number in hex | Easy       | Matrix exponentiation | addition, multiplication                                            |
