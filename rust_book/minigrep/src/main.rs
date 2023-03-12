use std::env;
use std::process;

use minigrep::Config;

/// IGNORE_CASE=1 cargo run -- yOu poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // let (query, file_path) = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \"{}\":", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        println!("Should have been able to read the file - Application error: {e}");
        process::exit(1);
    }
}




