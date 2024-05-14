use std::env;
use std::process;

use minigrep_lib::{run, Config};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
