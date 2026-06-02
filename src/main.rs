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

    let config = Config::new(&args);

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
    /// Creates a new `Config` instance from command line arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A slice of strings containing the command line arguments
    ///            where `args[0]` is the program name, `args[1]` is the search
    ///            query and `args[2]` is the file path.
    ///
    /// # Returns
    ///
    /// A new `Config` instance with owned `query` and `file_path` strings.
    ///
    /// # Panics
    ///
    /// Panics if fewer than 3 arguments are provided.
    ///
    /// # Example
    ///
    /// ```
    /// let args = vec![
    ///     String::from("program"),
    ///     String::from("hello"),
    ///     String::from("hello.txt"),
    /// ];
    ///
    /// let config = Config::new(&args);
    /// assert_eq!(config.query, "hello");
    /// assert_eq!(config.file_path, "hello.txt");
    /// ```
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
