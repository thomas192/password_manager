use std::env;
use std::process;

use password_manager::config::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = password_manager::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
