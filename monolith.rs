// This is a comment, and is ignored by the compiler.

// This code is editable, feel free to hack it!

// This is the main function.

mod display_mods;
use display_mods::{Groupable, wait_one_millis_and_micros_and_nanos, display_time_elapsed_nice, record_nanos};

fn main() {
    let duration_since_epoch_nanos = record_nanos();
    // Statements here are executed when the compiled binary is called.

    println!("welcome to a {} nanoseconds of runtime", 100000000.group_with_nothing());

    wait_one_millis_and_micros_and_nanos();
    display_time_elapsed_nice(duration_since_epoch_nanos);
}
