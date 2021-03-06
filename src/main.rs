use std::{
    env, 
    process, 
};
use minigrep::Config;

fn main() {
    println!("Minigrep!!!!!");
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for: {}, in file: {}\n", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}




