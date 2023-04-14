use clap::Parser;
use ansi_term::{Colour,Style};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MyArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    parse_args();
    coloured_and_styled();
}

fn parse_args(){
    let args = MyArgs::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

fn coloured_and_styled(){
    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Red.bold().paint("this is bold and colored"));
}

