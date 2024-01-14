use std::{env, process};

use rust_book_minigrep::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else( |err| {
        eprintln!("Error while parsing args {}",err);
        process::exit(1);
    });

    if let Err(e) = rust_book_minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


