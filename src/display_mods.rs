use std::time::{Duration, SystemTime};
use std::thread;

pub trait Groupable {
    fn group_with_nothing(&self) -> String;
}

impl Groupable for u128 {
    fn group_with_nothing(&self) -> String {
        self
        .to_string()                             // lol
        .as_bytes()                              // this is 
        .rchunks(3)                              // how
        .rev()                                   // we 
        .map(std::str::from_utf8)                // format large numbers
        .collect::<Result<Vec<&str>, _>>()       // to visually readable formats
        .unwrap()                                // in rust
        .join(" ")                               // and nobody minds this
    }
}

pub fn record_nanos() -> u128 {
	return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
}

pub fn wait_one_millis_and_micros_and_nanos(){
    let tsn = Duration::from_nanos(1);
    // Print text to the console.
    
    thread::sleep(tsn);

    let tsn2 = Duration::from_micros(1);
    // Print text to the console.
    
    thread::sleep(tsn2);

    let tsn3 = Duration::from_millis(1);
    // Print text to the console.
    
    thread::sleep(tsn3);
}

pub fn display_time_elapsed_nice(recorded_start: u128) {
    println!("Time elapsed since received timestamp: {}", (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() - recorded_start)
    .group_with_nothing()
    );                                       // this is great
}