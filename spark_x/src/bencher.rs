use colored::*;
use std::str;

pub struct Duration {
    pub microseconds: f64,
}

pub struct BenchmarkResults {
    pub average: Duration,
    pub min: Duration,
    pub max: Duration,
}

pub struct Bencher<'a> {
    executable: &'a str,
    iterations: u32,
    format: &'a str,
    results: Vec<Duration>,
}

impl<'a> Bencher<'a> {
    pub fn new(executable: &'a str, iterations: u32, format: &'a str) -> Self {
        
        Bencher {
            executable,
            iterations,
            format,
            results: Vec::with_capacity(iterations as usize),
        }
    }

    pub fn run_benchmark(&mut self) -> Option<BenchmarkResults> {
        use std::process::Command;

        let mut executable_tokens = self.executable.split_whitespace();
        let program_name = executable_tokens.next().expect("No program name").clone();
        let args = executable_tokens.collect::<Vec<&str>>();

        for _ in 0 .. self.iterations {
            let process_output = match Command::new(program_name)
                .args(&args)
                .output()
            {
                Ok(process) => { process },
                Err(err) => panic!(err),
            };

            let measure_output = str::from_utf8(&process_output.stdout).unwrap();
            let length = measure_output.len();
            let microseconds = &measure_output[0..length - 1].parse::<f64>().unwrap();

            self.results.push(Duration { microseconds: *microseconds });
        }

        Some(BenchmarkResults::new(&self.results))
    }

    pub fn print_formatted_results(&self, results: &BenchmarkResults) {
        if self.format == "csv" {
            println!("{};{};{}", results.average.pretty(), results.min.pretty(), results.max.pretty());
        }
        else {
            let avg_ms = results.average.microseconds;
            let min_ms = results.min.microseconds;
            let max_ms = results.max.microseconds;

            println!("");
            println!("{}", "Benchmark results: ".green().bold());
            println!("----------------------------------------");
            println!("{}", format!("Average time: {:.3} mircoseconds", avg_ms));
            println!("{}", format!("Minimum time: {:.3} mircoseconds", min_ms));
            println!("{}", format!("Maximum time: {:.3} mircoseconds", max_ms));
        
        }
    }
}

impl BenchmarkResults {
    fn new(measurements: &Vec<Duration>) -> BenchmarkResults {
        use std::f64;

        let mut results: (f64, f64, f64) = (0.0, f64::MAX, f64::MIN);

        for idx in 0 .. measurements.len() {
            let dur = measurements[idx].microseconds;

            results.0 += dur;
            
            if dur < results.1 { results.1 = dur } 
            if dur > results.2 { results.2 = dur } 
        }

        results.0 /= measurements.len() as f64;

        BenchmarkResults {
            average: Duration::new(results.0),
            min: Duration::new(results.1),
            max: Duration::new(results.2),
        }
    }
}

impl Duration {
    pub fn new(microseconds: f64) -> Self {
        Duration { microseconds }
    }

    pub fn pretty(&self) -> String {        
        format!("{:.3}", self.microseconds)
    }
}
