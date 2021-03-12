# bigint-benchmark-rs

A benchmark for Rust big integer implementations.

The benchmark is: calculate 1 million digits of the number e, using the Binary Splitting method.

Current results (on an Intel i7 laptop, average of 3 runs):

| Library        | Version | Time (s) | Notes    |
| -------------- | --------| ------ | -------- |
| rug            | 1.11.0  | 0.271  | Uses [GMP](https://gmplib.org/) |
| rust-gmp       | 0.5.0   | 0.278  | Uses [GMP](https://gmplib.org/) |
| ibig           | 0.2.0   | 1.014  | Pure rust |
| ramp           | 0.5.9   | 12.469 | Rust and inline assembly (requires nightly toolchain) |
| num-bigint     | 0.4.0   | 30.970 | Pure rust |
