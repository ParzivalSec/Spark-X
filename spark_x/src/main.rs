extern crate getopts;

mod clock;

use std::env;
use getopts::Options;
use clock::Clock;

fn print_spark_x_usage() {
    println!("Spark-X : A benchmarking utility");
    println!("Usage: spark_x.exe ");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints usage informations");
    opts.optopt("f", "format", "specifies the output format (console/csv)", "FORMAT");
    opts.optopt("i", "iterations", "number of iterations to measure", "NUM_ITERATION");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => { m },
        Err(f)  => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        println!("spark-x.exe --it 1000 --f csv PATH_TO_EXECUTABLE");
        return;
    }

    let format = matches.opt_str("f").expect("No format provided!");
    let iterations = match matches.opt_str("i").unwrap().parse::<u32>() {
        Ok(integer) => { integer },
        _ => { panic!("Could not parse number of iterations") }
    };

    println!("Run EXE {} times and output with format: {}", iterations, format);

    return;
}