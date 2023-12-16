use std::{env, process};

use rust_book_minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else( |err| {
        println!("Error while parsing args {}",err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the {}", config.file_path);
    if let Err(e) = rust_book_minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}


