use std::env;
use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // don't accept wrong input
        if args.len() < 3 {
            return Err("Not Enough args\nusage greprs <query> <file-path>");
        }

        // let ignore = if let Ok(_) = env::var("IGNORE_CASE") {
        //     true
        // } else {
        //     false
        // };
        // equivalent =>
        let ignore = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: ignore,
        });
    }
}

// Box<dyn Error> to box different kinds of errors and know their underlying type at runtime
// Box do operation for us => convert any type that impl Error trait to Box<Error>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case == true {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        println!("🪵 [lib.rs:44]~ token ~ \x1b[0;32mline\x1b[0m = {}", line);
    }
    return Ok(());
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in content.split("\n") {
        if line.contains(query) {
            res.push(line);
        }
    }
    return res;
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res: Vec<&str> = Vec::new();
    for line in content.split("\n") {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
