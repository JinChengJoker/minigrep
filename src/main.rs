use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("query is {}", config.query);
    println!("file_path is {}", config.file_path);

    let content = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("file content:\n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}