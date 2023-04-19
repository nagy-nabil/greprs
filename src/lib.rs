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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    println!("{}", content);
    Ok(())
}
