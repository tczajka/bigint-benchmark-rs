use std::io::Write;
use std::time::{Duration, Instant};
use clap::{Arg, ArgAction, Command, command, value_parser};
use number::Number;

mod digits_of_e;
mod fib;
mod number;


fn main() {
    let argv = command!()
        .subcommand_required(true)
        .propagate_version(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("lib")
                .value_name("lib")
                .short('l')
                .long("lib")
                .help("Library to use")
                .value_parser(["dashu", "ibig", "malachite", "num-bigint", "rug"])
                .required(true)
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("task")
                .value_name("task")
                .short('t')
                .long("task")
                .help("Task to run")
                .value_parser(["e", "fib", "fib_hex"])
                .required(true),
        )
        .arg(
            Arg::new("n")
                .value_name("n")
                .short('n')
                .long("num")
                .help("Number of elements to calculate")
                .value_parser(value_parser!(u32))
                .required(true),
        )
        .subcommands([
            Command::new("print")
                .about("Print the answer and check that all libraries agree"),
            Command::new("benchmark")
                .about("Benchmark the libraries"),
        ])
        .get_matches();

    let mut libs: Vec<&str> = argv.get_many::<String>("lib").unwrap().map(|v| v.as_str()).collect();
    libs.sort();
    let task = argv.get_one::<String>("task").unwrap().as_str();
    let n = *argv.get_one::<u32>("n").unwrap();

    match argv.subcommand() {
        Some(("print", _)) => command_print(&libs, task, n),
        Some(("benchmark", _)) => command_benchmark(&libs, task, n),
        _ => unreachable!("subcommand required"),
    }
}

fn command_print(libs: &[&str], task: &str, n: u32) {
    let mut answer: Option<String> = None;
    for lib_name in libs {
        let a = run_task(lib_name, task, n);
        match &answer {
            None => {
                println!("Answer = {}", a);
                println!("Results:");
                println!("    {:<10} {:>10}", lib_name, "agrees");
                answer = Some(a);
            }
            Some(ans) => {
                if *ans == a {
                    println!("    {:<10} {:>10}", lib_name, "agrees");
                } else {
                    println!("    {:<10} {:>10}", lib_name, "disagrees");
                }
            }
        }
    }
}

fn command_benchmark(libs: &[&str], task: &str, n: u32) {
    let mut answer: Option<String> = None;
    let mut results: Vec<(&str, Duration)> = Vec::new();

    println!("Benchmarking:");
    for lib_name in libs {
        print!("    {:<10}", lib_name);
        std::io::stdout().flush().unwrap();

        // Run benchmark for 60 seconds, or at least 5 iterations
        // Take the minimum duration of all iterations as the result

        let mut min_duration = Duration::MAX;
        let mut iterations: u64 = 0;

        let lib_start_time = Instant::now();
        let limit = Duration::from_secs(60);

        while lib_start_time.elapsed() < limit || iterations < 5 {
            let start_time = Instant::now();

            let a = run_task(lib_name, task, n);
            match &answer {
                None => answer = Some(a),
                Some(ans) => assert_eq!(*ans, a),
            }

            let elapsed = start_time.elapsed();
            if elapsed < min_duration {
                min_duration = elapsed;
            }

            iterations += 1;
        }

        println!(" ({} iterations)", iterations);

        results.push((lib_name, min_duration));
    }

    println!("Results:");
    results.sort_by_key(|&(_, d)| d);
    for (lib_name, duration) in results {
        println!("    {:<10} {} s", lib_name, duration.as_secs_f64());
    }
}

fn run_task(lib: &str, task: &str, n: u32) -> String {
    match lib {
        "dashu" => run_task_using::<dashu::Natural>(task, n),
        "ibig" => run_task_using::<ibig::UBig>(task, n),
        "malachite" => run_task_using::<malachite::natural::Natural>(task, n),
        "num-bigint" => run_task_using::<num_bigint::BigUint>(task, n),
        "rug" => run_task_using::<rug::Integer>(task, n),
        _ => unreachable!(),
    }
}

fn run_task_using<T: Number>(task: &str, n: u32) -> String {
    match task {
        "e" => digits_of_e::calculate::<T>(n),
        "fib" => fib::calculate_decimal::<T>(n),
        "fib_hex" => fib::calculate_hex::<T>(n),
        _ => unreachable!(),
    }
}
