# bigint-benchmark-rs

Benchmarks for Rust big integer implementations.

Each benchmark is run 5 times, with the calculation repeated for at least 10 seconds each time.
The median run is used as the result.

## Results

All benchmarks were run for n = 1 million and n = 10 million on an Intel i7 laptop.

| Library        | Version | Notes    | e 1mil | e 10mil | fib 1mil | fib 10mil | fib_hex 1mil | fib_hex 10mil |
| -------------- | --------| ------ | -------- | ------- | -------- | --------- | ------------ | ------------- |
| rug            | 1.11.0  | Uses [GMP](https://gmplib.org/) | e1mil | e 10mil | fib 1 mil | fib 10 mil | fib_hex 1mil | fib_hex 10mil |
| rust-gmp       | 0.5.0   | Uses [GMP](https://gmplib.org/) | e1mil | e 10mil | fib 10 mil | fib_hex 1mil | fib_hex 10mil |
| ibig           | 0.2.1   | Pure rust | e1mil | e 10mil | fib 10 mil | fib_hex 1mil | fib_hex 10mil |
| ramp           | 0.5.9   | Rust and inline assembly (requires nightly toolchain) | e1mil | e 10mil | fib 10 mil | fib_hex 1mil | fib_hex 10mil |
| num-bigint     | 0.4.0   | Pure rust | e1mil | e 10mil | fib 10 mil | fib_hex 1mil | fib_hex 10mil |

## Usage

Calculate 100 digits of `e` and compare answers:

```
$ bigint-benchmark --task e --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp -n 100 print                                             
answer = 2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427
ibig computed the answer
num-bigint agrees
ramp agrees
rug agrees
rust-gmp agrees
```

Calculate the 500-th Fibonacci number and compare answers:

```
$ ./target/release/bigint-benchmark --task fib --lib ibig --lib num-bigint --lib ramp --lib rug --lib rust-gmp -n 500 print                                            
answer = 139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125
ibig computed the answer
num-bigint agrees
ramp agrees
rug agrees
rust-gmp agrees
```

Benchmark calculating a million digits of e.

```
```

## Tasks

The tasks are:
* `e`: Calculate n digits of the number e, using the Binary Splitting method.
       This uses numbers of different sizes, addition, multiplication, division, and base
       conversion to 10.
* `fib`: Calculate the n-th Fibonacci number, using the matrix squaring method.
         This uses addition, multiplication and base conversion to 10.
* `fib_hex`: Calculate the n-th Fibonacci number in hexadecimal.
         This uses addition, multiplication and base conversion to 16.

`e` is the most interesting in that it uses the largest variety of operations and number sizes.
