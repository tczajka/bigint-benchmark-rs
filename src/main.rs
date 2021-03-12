use number::Number;
use std::time::{Duration, Instant};

mod digits_of_e;
mod number;

struct BenchmarkResult {
    library_name: &'static str,
    answer: String,
    time: Duration,
}

fn run_benchmark<T: Number>(library_name: &'static str) -> BenchmarkResult {
    eprintln!("Running {}", library_name);
    let start_time = Instant::now();
    let answer = digits_of_e::calculate::<T>();
    let time = start_time.elapsed();
    BenchmarkResult {
        library_name,
        answer,
        time,
    }
}

fn print_answers(results: &[BenchmarkResult]) {
    let top_result = results.first().unwrap();
    println!("e = {}", top_result.answer);
    for result in results {
        println!(
            "{:20} {:.3}s{}",
            result.library_name,
            result.time.as_secs_f64(),
            if result.answer == top_result.answer {
                ""
            } else {
                " disagrees!"
            }
        );
    }
}

fn main() {
    let mut results = Vec::new();

    results.push(run_benchmark::<ibig::UBig>("ibig 0.2.0"));
    results.push(run_benchmark::<num_bigint::BigUint>("num-bigint 0.4.0"));
    results.push(run_benchmark::<ramp::Int>("ramp 0.5.9"));
    results.push(run_benchmark::<rug::Integer>("rug 1.11.0"));
    results.push(run_benchmark::<gmp::mpz::Mpz>("rust-gmp 0.5.0"));

    results.sort_by_key(|result| result.time);
    print_answers(&results);
}
