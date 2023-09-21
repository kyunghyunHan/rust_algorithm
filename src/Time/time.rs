use std::time::{Instant};

fn main() {
    let start_time = Instant::now(); // Start timing
    
    let mut sum = 0u64;
    for i in 0..100_000_000_0 {
        sum += i;
    }
    
    let terminate_time = Instant::now(); // End timing
    
    let elapsed_time = terminate_time.duration_since(start_time);
    
    println!("{} seconds elapsed.", elapsed_time.as_secs());
}