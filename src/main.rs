use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    println!("query is {}", query);
    println!("file_path is {}", file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("file content:\n{content}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}