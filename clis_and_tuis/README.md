
# basics 
## grss
```bash
cargo add clap --features derive
...
env RUST_LOG=info cargo run clap Cargo.toml 
```
### [Cargo.toml](basics/grrs/Cargo.toml)
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
### [main.rs](basics/grrs/src/main.rs)
- imports
```rust
use clap::Parser;
use anyhow::{Context, Result};
use std::io::{Write};
use std::{thread, time::Duration};
use log::{info, warn};
```
- CLI arg parser
```rust
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
```
- main
```rust
// fn main() -> Result<(), Box<dyn std::error::Error>> {
// fn main() -> Result<(), CustomError> {
fn main() -> Result<()> {
    ...
    Ok(())
}
```
- grep functionality: read file from path and write out matched lines
```rust
// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");
let args = Cli::parse();
let content = std::fs::read_to_string(&args.path)
    //.map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.display(), err)))?;
    .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

// let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
let handle = std::io::stdout().lock();
find_matches(&content, &args.pattern, handle)?;
```
- progress bar:
```rust
let n = 10;
let pb = indicatif::ProgressBar::new(n);
for i in 0..n {
    thread::sleep(Duration::from_millis(n * 50));
    pb.println(format!("[+] finished #{}", i));
    pb.inc(1);
}
pb.finish_with_message("done");
```
- logger:
```rust
env_logger::init();
info!("shutting down");
warn!("blah!");
```
### [tests/cli.rs](basics/grrs/tests/cli.rs) - integration tests
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

## misc
### [Cargo.toml](basics/misc/Cargo.toml)
```toml
[dependencies]
anyhow = "1.0.81"
clap = { version = "4.5.4", features = ["derive"] }
config = "0.14.0"
crossbeam-channel = "0.5.12"
ctrlc = "3.4.4"
lazy_static = "1.4.0"
serde = {version="1.0.197", features = ["derive"]}
serde_derive = "1.0.197"
serde_json = "1.0.115"
signal-hook = "0.3.17"
```
### [main.rs](basics/misc/src/main.rs)
- imports:
```rust
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use anyhow::Result;
use config::{Config};
use std::collections::HashMap;
use clap::Parser;
use serde_json::json;
```
- config:
```rust
lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("settings"))
        .add_source(config::Environment::with_prefix("APP_NAME").separator("_"))
        .build()
        .unwrap();
}
pub fn get_from_config<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    // You shouldn't probably do it like that and actually handle that error that might happen
    // here, but for the sake of simplicity, we do it like this here
    CONFIG.get::<T>(key).unwrap()
}

...

 // Print out our settings (as a HashMap)
println!("{:?}",&<Config as Clone>::clone(&CONFIG).try_deserialize::<HashMap<String, String>>().unwrap());
let duration = get_from_config::<u64>("duration");
```
- signal handler with crossbeam:
```rust
fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

...

let ctrl_c_events = ctrl_channel()?;
let ticks = tick(Duration::from_secs(get_from_config::<u64>("duration")));
loop {
    select! {
        recv(ticks) -> _ => {
            println!("working!");
        }
        recv(ctrl_c_events) -> _ => {
            println!();
            println!("Goodbye!");
            break;
        }
    }
}
```
# clap
## [quickstart](clap/quickstart/src/main.rs)
```bash
cargo run -- --help
cargo run -- --name=bob --count=3
cargo run -- -n=bob -c=3
```
```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
```
## [derive](clap/derive/src/main.rs)
- `cargo run -- test --list` - subcommands
## [builder](clap/builder/src/main.rs)
- `cargo run -- test --list` - subcommands

# RataTUI
## hello-ratatui
- [Cargo.toml](ratatui/hello-ratatui/Cargo.toml)
```toml
[dependencies]
crossterm = "0.27.0"
ratatui = "0.26.1"

```
- [main.rs](ratatui/hello-ratatui/src/main.rs)
```rust
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // main loop
    loop {
        // draw the UI
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        // handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q'))
                {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
```