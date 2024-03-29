use clap::Parser;
use anyhow::{Context, Result};
use std::io::{Write};
use std::{thread, time::Duration};
use log::{info, warn};


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[allow(dead_code)]
#[derive(Debug)]
struct CustomError(String);

fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() -> Result<(), CustomError> {
fn main() -> Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        //.map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.display(), err)))?;
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    // let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    let handle = std::io::stdout().lock();
    find_matches(&content, &args.pattern, handle)?;


    let n = 10;
    let pb = indicatif::ProgressBar::new(n);
    for i in 0..n {
        thread::sleep(Duration::from_millis(n * 50));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    env_logger::init();
    info!("shutting down");
    warn!("blah!");

    Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}