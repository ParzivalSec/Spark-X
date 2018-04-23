extern crate getopts;
extern crate colored;

mod clock;
mod bencher;

use std::env;
use getopts::Options;
use colored::*;

use bencher::{ Bencher };

fn print_spark_x_usage() {
    println!("");
    println!("{}", "Spark-X : A benchmarking utility".bold());
    println!("{}", "Usage: spark_x.exe --i NUM_ITERATIONS --f csv|console PATH_TO_EXECUTABLE");
    println!("{}", "\nOptions:".bold());
    println!("");
    println!("{}", "-iterations (--i):    Number of iterations the executable's time is measured and averaged");
    println!("{}", "-format     (--f):    Format of the output results. Can either be csv or console");
    println!("{}", "-help       (--h):    Display usage hints");
}

fn shutdown_with_error(msg: &'static str) -> ! {
    println!("{} {}", "Error: ".red(), msg.red());
    panic!(msg);
}

fn validate_format(format: &str) {
    if format != "csv" && format != "console" {
        shutdown_with_error("Specified format is not supported! Please use 'csv' or 'console' instead");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints usage informations");
    opts.optopt("f", "format", "specifies the output format (console/csv)", "FORMAT");
    opts.optopt("i", "iterations", "number of iterations to measure", "NUM_ITERATION");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => { m },
        Err(f)  => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") || args.len() == 1 {
        print_spark_x_usage();
        return;
    }

    let format = match matches.opt_str("f") {
        Some(format) => { validate_format(&format); format },
        None => shutdown_with_error("No format provided!"),
    };

    let iterations = match matches.opt_str("i").unwrap().parse::<u32>() {
        Ok(integer) => { integer },
        _ => { shutdown_with_error("Could not parse number of iterations. Only postive numbers are allowed!") }
    };

    let executable = if matches.free.len() >= 1 {
        matches.free[0].clone()
    } else {
        shutdown_with_error("No executable path provided!");
    };

    let mut bencher = Bencher::new(&executable, iterations, &format);
    let results = bencher.run_benchmark().expect("Benchmarking failed!");
    bencher.print_formatted_results(&results);

    return;
}