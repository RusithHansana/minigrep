//! A simple grep-like command tool
//!
//! Usage: cargo run -- <query> <file_path>
//! Example: cargo run -- hello hello.txt
use std::env;
use std::fs;
use std::process;

fn main() {
    // Collects command line arguments as a vector of `String`
    // arg()[0] - is the program name
    // arg()[1] - is the search query
    // arg()[2] - is the file path
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem while parsing the arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    // Read the file contents
    // Returns: std::io::Result<String>
    let content = fs::read_to_string(config.file_path).expect("Error while reading the file!");

    println!("with content: \n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Expected a query and a file path");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
