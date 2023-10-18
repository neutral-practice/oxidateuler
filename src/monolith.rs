// This is a comment, and is ignored by the compiler.

// This code is editable, feel free to hack it!

// This is the main function.

mod display_mods;
use display_mods::{
    display_time_elapsed_nice, record_nanos, wait_one_millis_and_micros_and_nanos, Groupable,
};
mod window_mods;
use window_mods::window_function;

fn main() {
    let duration_since_epoch_nanos = record_nanos();
    // Statements here are executed when the compiled binary is called.

    // let warning_test = "unused"; // results in CI warning

    println!(
        "Welcome to a {} nanoseconds of runtime",
        100000000.group_with_nothing()
    );

    ///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\[ main Main main ]///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\
    ///|||\\\
    ///|||\\\
    ///|||\\\
    if cfg!(windows) {
        window_function();
    }

    ///\\\|||///
    ///\\\|||///
    ///\\\|||///
    ///\\\|||///\\\|||///\\\|||///\\\|||///\\\|||///[ the end of main Main main ]\\\|||///\\\|||///\\\|||///\\\|||///\\\|||///\\\|||///
    wait_one_millis_and_micros_and_nanos();
    return display_time_elapsed_nice(duration_since_epoch_nanos);
}
