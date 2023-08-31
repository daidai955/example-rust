use std::env;
use std::process;
use crate::grep_utils::{Config,read};

pub fn run() {
    println!("-------- grep.rs ---------");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = read(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
