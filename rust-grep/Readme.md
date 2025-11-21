# rust-grep
(A Minimal Grep Clone)

The Objective: Build a CLI tool that searches for a string in a file and prints the lines containing it. Primary Lesson: Understanding Ownership, Borrowing, and Lifetimes.

In C, you might return a char* pointing into a buffer. In Python, strings are just objects. In Rust, if your search function returns the text of a matching line, you must prove to the compiler that the returned reference (&str) does not outlive the source text (String).

## The MVP Requirements
- CLI Arguments: The user must provide a query (string) and a filename (path).
- File Reading: Read the file contents into memory.
- Logic: Iterate through lines; if a line contains the query, print it.
- Error Handling: If the file doesn't exist or args are missing, print a user-friendly error (no panics/crashes).

## The "Hard Part" (Where you will get stuck)

You will likely write a function that looks like this:
```
// This will likely fail to compile initially due to lifetime elision rules
fn search(query: &str, contents: &str) -> Vec<&str> { ... }
```

Rust needs to know: Does the returned &str refer to query or contents? You will learn to explicitly annotate lifetimes: `fn search<'a>(..., contents: &'a str) -> Vec<&'a str>`.

## Implementation Roadmap
1. Argument Parsing: Use `std::env::args()`. Collect them into a Vector or String.
2. File I/O: Use `std::fs::read_to_string`.
  - Contrast: In C, you'd `fopen`. In Rust, `read_to_string` handles the buffer allocation for you, but returns a Result.
3. The Logic:
  - Use the `.lines()` iterator on the content string.
  - Use `.contains()` to check for the query.

4. Refactoring (Crucial): Move your logic out of `main.rs` and into `lib.rs`. Separation of concerns is idiomatic in Rust.

4. Stretch Goals (To learn more)
  - Case Insensitive: Add an environment variable CASE_INSENSITIVE=1 to toggle this.
  - Standard Input: If no filename is provided, read from stdin (Unix pipe philosophy).
  - Color: Make the matching word red. This forces you to learn how to manipulate terminal output.
