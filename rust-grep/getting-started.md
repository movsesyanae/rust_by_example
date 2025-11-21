# Getting Started with rust-grep

This guide will walk you through compiling and running your `rust-grep` project.

## Prerequisites

Make sure you have Rust and Cargo installed. If you don't, you can install them by following the official instructions at [rustup.rs](https://rustup.rs/).

## Project Structure

The project is organized into a binary and a library crate, which is a common pattern in Rust:

- `src/main.rs`: This is the entry point of your command-line application. It's responsible for parsing command-line arguments and handling top-level errors.
- `src/lib.rs`: This is the library crate where the core logic of your application resides. It makes your code reusable and easy to test.
- `Cargo.toml`: This file is the manifest for your Rust project. It contains metadata and dependencies for your project.

## Building the Project

To compile your project, navigate to the project root directory in your terminal and run:

```bash
cargo build
```

This will compile your code and create an executable file in the `target/debug/` directory.

## Running the Tests

Your project comes with a test for the `search` function in `src/lib.rs`. To run the tests, use:

```bash
cargo test
```

All tests should pass. If they don't, something might be wrong with your implementation of the `search` function.

## Running the Application

To run your `rust-grep` application, you'll need a file to search in. Let's create a sample file named `poem.txt`:

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
```

Save this content in a file named `poem.txt` in the root of your project directory.

Now, you can run your program with `cargo run`, passing the search query and the filename as arguments. Note that arguments passed after `--` are sent to your program, not to Cargo.

```bash
cargo run -- nobody poem.txt
```

You should see the following output:

```
Searching for 'nobody' in 'poem.txt'
I'm nobody! Who are you?
Are you nobody, too?
```

### Running the optimized release build

When you are ready to release your application, you can build an optimized version with:

```bash
cargo build --release
```

The executable will be in `target/release/rust-grep`. You can run it directly:

```bash
./target/release/rust-grep nobody poem.txt
```

This will be much faster than the debug build.

Good luck, and have fun learning Rust!
