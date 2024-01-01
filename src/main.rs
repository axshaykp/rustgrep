use std::env;
use std::process;
use rustgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("query: {:?}, filename: {:?}", config.query, config.filename);

    if let Err(e) = rustgrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}


