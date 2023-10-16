// This is a comment, and is ignored by the compiler.

// This code is editable, feel free to hack it!

// This is the main function.

use std::time::{Duration, SystemTime};

use std::thread;


fn main() {
    let duration_since_epoch_nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    // Statements here are executed when the compiled binary is called.

    wait_one_millis_and_micros_and_nanos();
    display_time_elapsed_nice(duration_since_epoch_nanos);
}

fn wait_one_millis_and_micros_and_nanos(){
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

fn display_time_elapsed_nice(recorded_start: u128) {
    println!("Hello World! Time passed {}", (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() - recorded_start)
    .to_string()                             // lol
    .as_bytes()                              // this is 
    .rchunks(3)                              // how
    .rev()                                   // we 
    .map(std::str::from_utf8)                // format large numbers
    .collect::<Result<Vec<&str>, _>>()       // to visually readable formats
    .unwrap()                                // in rust
    .join(" ")                               // and nobody minds this
    );                                       // this is great
}