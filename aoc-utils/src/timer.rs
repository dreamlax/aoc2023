use std::time::Duration;
use std::time::Instant;

pub struct Timer(pub Instant);

impl Timer {
    pub fn new() -> Self{
        Self(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("Execution time: {}µs", self.0.elapsed().as_micros())
    }
}
