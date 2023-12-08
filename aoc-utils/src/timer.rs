use std::time::Instant;

pub struct Timer {
    start: Instant
}

impl Timer {
    pub fn new() -> Self{
        Self {
            start: Instant::now()
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("Execution time: {}µs", self.start.elapsed().as_micros())
    }
}
