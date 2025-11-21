use std::env;
use std::process;

use rust_grep::Config;

/// The main entry point of the application.
///
/// This function is responsible for:
/// 1. Collecting command-line arguments.
/// 2. Parsing those arguments into a `Config` struct.
/// 3. Handling any errors during parsing by printing a message to stderr
///    and exiting.
/// 4. Calling the `run` function from the library crate with the `Config`.
/// 5. Handling any errors returned by `run` and exiting.
fn main() {
    // `env::args()` returns an iterator over the command-line arguments.
    // `.collect()` turns that iterator into a `Vec<String>`.
    let args: Vec<String> = env::args().collect();

    // Parse arguments, handling errors with `unwrap_or_else`.
    // This is a clean way to provide custom error handling for a `Result`.
    // If `Config::new` returns an `Ok`, the `Config` value is unwrapped.
    // If it returns an `Err`, the closure is executed.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // Print the error message to the standard error stream.
        eprintln!("Problem parsing arguments: {}", err);
        // Exit the program with a non-zero status code to indicate an error.
        process::exit(1);
    });

    // `if let` provides a concise way to handle a `Result`.
    // If `run` returns an `Err(e)`, this block is executed.
    if let Err(e) = rust_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
