use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    loop {
        let now = Instant::now();
        println!("test");
        sleep(Duration::from_millis(17u64  - now.elapsed().as_millis() as u64));
    }
}
