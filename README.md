# bigint-benchmark-rs

Benchmarks for Rust big integer implementations.

Benchmark for each library is run 5 times for at least 10 seconds.
Result for each run is obtained by dividing the time it took to finish by the number of iterations in that run.
The fastest of the 5 runs is used as the result.

## Libraries

| Library                                           | Version | License           | Notes                                                                                    |
|---------------------------------------------------|---------|-------------------|------------------------------------------------------------------------------------------|
| [dashu](https://crates.io/crates/dashu)           | 0.3.1   | MIT or Apache 2.0 | Pure Rust, no_std                                                                        |
| [ibig](https://crates.io/crates/ibig)             | 0.3.6   | MIT or Apache 2.0 | Pure Rust, no_std                                                                        |
| [malachite](https://crates.io/crates/malachite)   | 0.4.0   | LGPL 3.0          | Pure Rust, derived from [GMP](https://gmplib.org) and [FLINT](https://www.flintlib.org/) |
| [num-bigint](https://crates.io/crates/num-bigint) | 0.4.4   | MIT or Apache 2.0 | Pure Rust, no_std                                                                        |
| [rug](https://crates.io/crates/rug)               | 1.21.0  | LGPL 3.0          | Links to [GMP](https://gmplib.org/)                                                      |

## Results

| Library                                           | e 100k |  e 1m |   e 10m | fib 10m | fib 100m | fib_hex 100m |
|---------------------------------------------------|-------:|------:|--------:|--------:|---------:|-------------:|
| [rug](https://crates.io/crates/rug)               |  0.014 | 0.268 |   4.493 |   0.304 |    5.367 |        0.949 |
| [malachite](https://crates.io/crates/malachite)   |  0.043 | 0.929 |  15.798 |   1.197 |   21.473 |        3.270 |
| [ibig](https://crates.io/crates/ibig)             |  0.031 | 0.978 |  31.606 |   1.252 |   40.075 |        8.485 |
| [num-bigint](https://crates.io/crates/num-bigint) |  0.086 | 6.776 | 656.515 |  10.271 |  950.146 |        8.967 |
| [dashu](https://crates.io/crates/dashu)           |  x.xxx | x.xxx |   x.xxx |   x.xxx |    x.xxx |        x.xxx |

Times in seconds.

## Usage

Calculate 100 digits of `e` and compare answers:
```
$ bigint-benchmark --task e --lib dashu --lib ibig --lib malachite --lib num-bigint --lib rug -n 100 print
Answer = 2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427
Results:
    dashu          agrees
    ibig           agrees
    malachite      agrees
    num-bigint     agrees
    rug            agrees
```

Calculate the 500-th Fibonacci number and compare answers:
```
$ bigint-benchmark --task fib --lib dashu --lib ibig --lib malachite --lib num-bigint --lib rug -n 500 print
Answer = 139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125
Results:
    dashu          agrees
    ibig           agrees
    malachite      agrees
    num-bigint     agrees
    rug            agrees
```

Benchmark calculating a million digits of e:
```
$ bigint-benchmark --task e --lib dashu --lib ibig --lib malachite --lib num-bigint --lib rug -n 1000000 benchmark
```

## Tasks

| Task      | Description                  | Difficulty | Algorithm             | Operations                                                          |
|-----------|------------------------------|------------|-----------------------|---------------------------------------------------------------------|
| `e`       | n digits of e                | Hard       | Binary splitting      | addition, multiplication, division, exponentiation, base conversion |
| `fib`     | n-th Fibonnaci number        | Medium     | Matrix exponentiation | addition, multiplication, base conversion                           |
| `fib_hex` | n-th Fibonnaci number in hex | Easy       | Matrix exponentiation | addition, multiplication                                            |
