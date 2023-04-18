use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // don't accept wrong input
    if args.len() < 3 {
        panic!("usage greprs <string to search> <file path>");
    }
    let search_key = &args[1];
    let file_path = &args[2];
    let content = fs::read_to_string(file_path).expect("cannot read this file");
    println!(
        "🪵 [main.rs:12]~ token ~ \x1b[0;32mcontent\x1b[0m = {}",
        content
    );
}
