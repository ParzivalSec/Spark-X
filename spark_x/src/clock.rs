extern crate time;

pub struct Clock {
    start_ns: u64, 
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            start_ns: 0,
        }
    }

    pub fn start(&mut self) {
        self.start_ns = time::precise_time_ns();
    }

    pub fn get_elapsed_time_ns(&self) -> u64 {
        let current_time = time::precise_time_ns();
        let elapsed_time = current_time - self.start_ns;
        elapsed_time
    }

    pub fn get_elapsed_time_ms(&self) -> u64 {
        let current_time = time::precise_time_ns();
        let elapsed_time = current_time - self.start_ns;
        elapsed_time / 1000000
    }

    pub fn get_elasped_time_s_ms_ns(&self) -> (u64, u64, u64) {
        let current_time = time::precise_time_ns();
        let elapsed_time_ns = current_time - self.start_ns;
        
        let ns_wihout_seconds = elapsed_time_ns % 1000000000;
        let seconds = (elapsed_time_ns - ns_wihout_seconds) / 1000000000;
        let ns_without_ms = ns_wihout_seconds % 1000000;
        let milliseconds = (ns_wihout_seconds - ns_without_ms) / 1000000;
        
        (seconds, milliseconds, ns_without_ms)
    }

    pub fn reset(&mut self) {
        self.start_ns = 0;
    }
}
