use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("query is {}", config.query);
    println!("file_path is {}", config.file_path);

    let content =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("file content:\n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
