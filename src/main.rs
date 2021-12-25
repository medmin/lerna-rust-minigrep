use std::{env, process};

use minigrep::Config;


fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Config Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
