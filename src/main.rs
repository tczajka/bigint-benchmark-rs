use clap::{command, value_parser, Arg, ArgAction, Command};
use number::Number;
use std::time::{Duration, Instant};

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
                .action(ArgAction::Append),
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
            Command::new("print").about("Print the answer and check that all libraries agree"),
            Command::new("benchmark").about("Benchmark the libraries"),
        ])
        .get_matches();

    let libs: Vec<&str> = argv
        .get_many::<String>("lib")
        .unwrap()
        .map(|v| v.as_str())
        .collect();
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
        let (a, _) = run_task(lib_name, task, n, 1);
        match &answer {
            None => {
                println!("answer = {}", a);
                println!("{:10} agrees", lib_name);
                answer = Some(a);
            }
            Some(ans) => {
                if *ans == a {
                    println!("{:10} agrees", lib_name);
                } else {
                    println!("{} disagrees!", lib_name);
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
        println!("    {:<10}", lib_name);

        // Take the median of 5 attempts, each attempt at least 10 seconds.
        let mut durations: Vec<Duration> = Vec::new();
        for sample_number in 1..=5 {
            let mut iter = 0;
            let mut duration = Duration::from_secs(0);
            while duration < Duration::from_secs(10) {
                let i = iter.max(1);
                let (a, d) = run_task(lib_name, task, n, i);
                match &answer {
                    None => answer = Some(a),
                    Some(ans) => assert_eq!(*ans, a),
                }
                iter += i;
                duration += d;
            }
            let duration = duration / iter;
            println!(
                "        attempt {}: {} ms ({} iteration{})",
                sample_number,
                iter,
                duration.as_millis(),
                if iter == 1 { "" } else { "s" }
            );
            durations.push(duration);
        }
        durations.sort();
        let duration = durations[0];
        results.push((lib_name, duration));
    }
    results.sort_by_key(|&(_, d)| d);
    println!("Results:");
    for (lib_name, duration) in results {
        println!("    {:<10} {} ms", lib_name, duration.as_millis());
    }
}

fn run_task(lib: &str, task: &str, n: u32, iter: u32) -> (String, Duration) {
    match lib {
        "ibig" => run_task_using::<ibig::UBig>(task, n, iter),
        "num-bigint" => run_task_using::<num_bigint::BigUint>(task, n, iter),
        "rug" => run_task_using::<rug::Integer>(task, n, iter),
        "malachite" => run_task_using::<malachite::natural::Natural>(task, n, iter),
        "dashu" => run_task_using::<dashu::Natural>(task, n, iter),
        _ => unreachable!(),
    }
}

fn run_task_using<T: Number>(task: &str, n: u32, iter: u32) -> (String, Duration) {
    let mut answer = None;
    let start_time = Instant::now();
    for _ in 0..iter {
        let a = match task {
            "e" => digits_of_e::calculate::<T>(n),
            "fib" => fib::calculate_decimal::<T>(n),
            "fib_hex" => fib::calculate_hex::<T>(n),
            _ => unreachable!(),
        };
        match &answer {
            None => answer = Some(a),
            Some(ans) => assert!(a == *ans),
        }
    }
    let time = start_time.elapsed();
    (answer.unwrap(), time)
}
