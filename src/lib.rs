use std::env;
use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // first index contains app name so skip it
        args.next();

        let query = args.next().expect("provide query");
        let file_path = args.next().expect("provide file_path");
        // let ignore = if let Ok(_) = env::var("IGNORE_CASE") {
        //     true
        // } else {
        //     false
        // };
        // equivalent =>
        let ignore = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query,
            file_path,
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
    return content.split("\n").filter(|v| v.contains(query)).collect();
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    return content
        .lines()
        .filter(|v| v.to_lowercase().contains(&query))
        .collect();
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
