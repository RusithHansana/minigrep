//! A simple grep-like command tool
//!
//! Usage: cargo run -- <query> <file_path>
//! Example: cargo run -- hello hello.txt
use std::env;

fn main() {
    // Collects command line arguments as a vector of `String`
    // arg()[0] - is the program name
    // arg()[1] - is the search query
    // arg()[2] - is the file path
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
