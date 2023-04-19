use greprs::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("🪵 [main.rs:5]~ token ~ \x1b[0;32margs\x1b[0m = {:?}", args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("error while running {}", e);
        process::exit(1);
    }
}
