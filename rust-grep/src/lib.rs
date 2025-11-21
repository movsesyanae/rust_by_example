use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// Parses command-line arguments and builds a `Config` instance.
    ///
    /// # Inputs
    ///
    /// * `args`: A slice of `String`s representing the command-line arguments.
    ///   The first element (`args[0]`) is the program name.
    ///
    /// # Outputs
    ///
    /// * A `Result` containing a `Config` on success or a static string error
    ///   message on failure.
    ///
    /// # Hint
    ///
    /// Check if `args` has at least 3 elements. If not, return an `Err`.
    /// The query will be `args[1]` and the filename will be `args[2]`.
    /// You'll need to `.clone()` the values from the slice to take ownership.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

/// The primary logic for the command-line tool.
///
/// # Inputs
///
/// * `config`: A `Config` struct holding the query and filename.
///
/// # Outputs
///
/// * A `Result` that is empty on success (`()`) or contains a `Box<dyn Error>`
///   on failure. This flexible error type allows returning different kinds of
///   errors (e.g., I/O errors).
///
/// # Hint
///
/// 1. Use `fs::read_to_string` to read the file specified in `config.filename`.
///    The `?` operator is useful here for propagating errors.
/// 2. Call the `search` function with the query and file contents.
/// 3. Iterate over the results from `search` and print each line.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Searching for '{}' in '{}'", config.query, config.filename);

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

/// Searches for `query` in `contents`, returning lines that contain it.
///
/// This is the core logic where you'll encounter Rust's lifetime rules.
///
/// # Inputs
///
/// * `query`: The string slice to search for.
/// * `contents`: The string slice of content to search within.
///
/// # Outputs
///
/// * A `Vec<&'a str>`: A vector of string slices, where each slice is a
///   reference to a line in the *original* `contents` that contains the `query`.
///
/// # Lifetime Hint
///
/// The lifetime parameter `'a` on the function signature is crucial. It tells
/// the Rust compiler that the returned vector of string slices contains
/// references that are tied to the lifetime of the `contents` slice.
/// This guarantees that the references will not outlive the data they point to.
///
/// # Implementation Hint
///
/// 1. Create an empty mutable `Vec` to store results.
/// 2. Iterate over each `line` in `contents` using the `.lines()` method.
/// 3. For each `line`, use the `.contains()` method to see if it includes the `query`.
/// 4. If it does, push the `line` to your results vector.
/// 5. Return the results vector.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
