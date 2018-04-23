use colored::*;

pub struct Duration {
    pub nanoseconds: u64,
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
        use clock::Clock;

        let mut clock = Clock::new();
        for it in 0 .. self.iterations {
            clock.start();

            let process_output = match Command::new(self.executable).output() {
                Ok(process) => { process },
                Err(err) => panic!(err),
            };

            self.results.push(Duration {
                nanoseconds: clock.get_elapsed_time_ns(),
            });
        
            clock.reset();
        }

        Some(BenchmarkResults::new(&self.results))
    }

    pub fn print_formatted_results(&self, results: &BenchmarkResults) {
        if self.format == "csv" {
            println!("{};{};{}", results.average.pretty(), results.min.pretty(), results.max.pretty());
        }
        else {
            let (avg_m, avg_s, avg_ms) = results.average.as_unit_tuple();
            let (min_m, min_s, min_ms) = results.min.as_unit_tuple();
            let (max_m, max_s, max_ms) = results.max.as_unit_tuple();

            println!("");
            println!("{}", "Benchmark results: ".green().bold());
            println!("----------------------------------------");
            println!("{}", format!("Average time: {} minutes, {} seconds, {} milliseconds", avg_m, avg_s, avg_ms));
            println!("{}", format!("Minimum time: {} minutes, {} seconds, {} milliseconds", min_m, min_s, min_ms));
            println!("{}", format!("Maximum time: {} minutes, {} seconds, {} milliseconds", max_m, max_s, max_ms));
        }
    }
}

impl BenchmarkResults {
    fn new(measurements: &Vec<Duration>) -> BenchmarkResults {
        use std::u64;

        let mut results: (u64, u64, u64) = (0, u64::MAX, u64::MIN);

        for idx in 0 .. measurements.len() {
            let dur = measurements[idx].nanoseconds;

            results.0 += dur;
            
            if dur < results.1 { results.1 = dur } 
            if dur > results.2 { results.2 = dur } 
        }

        results.0 /= measurements.len() as u64;

        BenchmarkResults {
            average: Duration::new(results.0),
            min: Duration::new(results.1),
            max: Duration::new(results.2),
        }
    }
}

impl Duration {
    pub fn new(nanoseconds: u64) -> Self {
        Duration { nanoseconds }
    }

    pub fn as_unit_tuple(&self) -> (u64, u64, u64) {
        let mut total_ns = self.nanoseconds;

        let ns_without_minutes = total_ns % 60000000000;
        let minutes = (total_ns - ns_without_minutes) / 60000000000;
        total_ns = ns_without_minutes;

        let ns_without_seconds = total_ns % 1000000000;
        let seconds = (total_ns - ns_without_seconds) / 1000000000;
        total_ns = ns_without_seconds;

        let ns_without_milliseconds = total_ns % 1000000;
        let milliseconds = (total_ns - ns_without_milliseconds) / 1000000;

        (minutes, seconds, milliseconds)
    }

    pub fn pretty(&self) -> String {        
        let (minutes, seconds, milliseconds) = self.as_unit_tuple();
        format!("{}:{}.{}", minutes, seconds, milliseconds)
    }
}
