use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use anyhow::Result;
use config::{Config};
use std::collections::HashMap;
use clap::Parser;
use serde_json::json;

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

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

#[derive(Parser)]
struct Cli {
    /// Output JSON instead of human readable messages
    #[arg(long = "json", default_value = "true")] // cargo run  -- --json
    json: bool,
}

fn main() -> Result<()> {

    let args = Cli::parse();
    if args.json {
        println!("{}",
            json!({
                "type": "message",
                "content": "blah",
            })
        );
    }

    // Print out our settings (as a HashMap)
    println!("{:?}",&<Config as Clone>::clone(&CONFIG).try_deserialize::<HashMap<String, String>>().unwrap());

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

    Ok(())
}