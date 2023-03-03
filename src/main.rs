use std::{env, process};

fn main() {
    let config = minigrep::Config::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("prase args: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("error: {err}");
        process::exit(1);
    }
}
