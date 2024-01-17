use std::time::{Duration, Instant};

fn main() {

    for num in 1..100 {
        //let ts = minstant::Instant::now();
        let ts = Instant::now();
        std::thread::park_timeout(Duration::from_micros(num));
        let elapsed = ts.elapsed();
        println!("{num} Elapsed: {elapsed:?} | {}", minstant::is_tsc_available());
    }

}