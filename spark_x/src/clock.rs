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

    pub fn reset(&mut self) {
        self.start_ns = 0;
    }
}
