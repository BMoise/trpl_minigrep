use std::env;
use std::process;

use trpl_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });

    if let Err(e) = trpl_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
