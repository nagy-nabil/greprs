use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // don't accept wrong input
        if args.len() < 3 {
            return Err("Not Enough args\nusage greprs <query> <file-path>");
        }

        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        });
    }
}

// Box<dyn Error> to box different kinds of errors and know their underlying type at runtime
// Box do operation for us => convert any type that impl Error trait to Box<Error>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &content) {
        println!("🪵 [lib.rs:27]~ token ~ \x1b[0;32mline\x1b[0m = {}", line);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
