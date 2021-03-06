use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("filename {}", config.filename);
    let contents = fs::read_to_string(config.filename).expect("Error reading file");

    println!("content {}", contents);
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
