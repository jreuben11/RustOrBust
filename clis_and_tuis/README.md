
# basics 
## grss
```bash
cargo add clap --features derive
...
env RUST_LOG=info cargo run clap Cargo.toml 
```
- [](basics/grrs/Cargo.toml)
```toml
[dependencies]
anyhow = "1.0.81"
clap = { version = "4.5.4", features = ["derive"] }
env_logger = "0.11.3"
indicatif = "0.17.8"
log = "0.4.21"

[dev-dependencies]
assert_cmd = "2.0.14"
assert_fs = "1.1.1"
predicates = "3.1.0"
```
- [](basics/grrs/src/main.rs)
```rust
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

// fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() -> Result<(), CustomError> {
fn main() -> Result<()> {
    ...
    Ok(())
}

// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");
let args = Cli::parse();
let content = std::fs::read_to_string(&args.path)
    //.map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.display(), err)))?;
    .with_context(|| format!("could not read file `{}`", &args.path.display()))?;


// let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
let handle = std::io::stdout().lock();
find_matches(&content, &args.pattern, handle)?;

// progress bar:
let n = 10;
let pb = indicatif::ProgressBar::new(n);
for i in 0..n {
    thread::sleep(Duration::from_millis(n * 50));
    pb.println(format!("[+] finished #{}", i));
    pb.inc(1);
}
pb.finish_with_message("done");

// logger:
env_logger::init();
info!("shutting down");
warn!("blah!");
```
- [](basics/grrs/tests/cli.rs) - integration tests
```rust
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use assert_fs::prelude::*;
...
let file = assert_fs::NamedTempFile::new("sample.txt")?;
file.write_str("A test\nActual content\nMore content\nAnother test")?;

let mut cmd = Command::cargo_bin("grrs")?;
cmd.arg("test").arg(file.path());
cmd.assert()
    .success()
    .stdout(predicate::str::contains("test\nAnother test"));
```