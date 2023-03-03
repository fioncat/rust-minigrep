use std::{env, fs, io::Error};

pub struct Config {
    file_path: String,
    query: String,
    ignore_case: bool,
}

impl Config {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        // ignore the first program name arg.
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("missing file_path arg")),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("missing query arg")),
        };

        let ignore_case = match args.next() {
            Some(arg) => match arg.as_str() {
                "--ignore-case" | "-i" => true,
                _ => return Err(format!("invalid arg {arg}")),
            },
            None => env::var("IGNORE_CASE").is_ok(),
        };

        return Ok(Config {
            file_path,
            query,
            ignore_case,
        });
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search(
            &content,
            config.query.to_uppercase().as_str(),
            config.ignore_case,
        )
    } else {
        search(&content, &config.query, config.ignore_case)
    };

    for item in results {
        println!("{item}");
    }

    return Ok(());
}

fn search<'a>(content: &'a str, query: &str, ignore_case: bool) -> Vec<&'a str> {
    return content
        .lines()
        .filter(|line| {
            if ignore_case {
                return line.to_uppercase().contains(query);
            }
            return line.contains(query);
        })
        .collect();
}
