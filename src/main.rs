use std::env;
use std::process;

mod config;
use config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error {err}");
        process::exit(1);
    });

    println!("{:?}", config);
}
