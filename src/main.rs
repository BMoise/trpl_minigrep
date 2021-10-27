use std::env;
use std::process;

use trpl_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = trpl_minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
