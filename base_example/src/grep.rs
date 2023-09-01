use crate::grep_utils::{read, Config};
use std::env;
use std::process;

pub fn run() {
    println!("-------- grep.rs ---------");

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = read(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
