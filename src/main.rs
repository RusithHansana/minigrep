//! A simple grep-like command tool
//!
//! Usage: cargo run -- <query> <file_path>
//! Example: cargo run -- hello hello.txt
use std::env;
use std::fs;

fn main() {
    // Collects command line arguments as a vector of `String`
    // arg()[0] - is the program name
    // arg()[1] - is the search query
    // arg()[2] - is the file path
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query} in {file_path}");

    // Read the file contents
    // Returns: std::io::Result<String>
    let content = fs::read_to_string(file_path).expect("Error while reading the file!");

    println!("with content: \n{content}");
}
