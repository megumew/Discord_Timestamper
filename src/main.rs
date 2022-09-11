use chrono::*;
use clap::{Parser, Subcommand};
extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

/// A Discord Timestamp Generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long, action)]
        list: bool,
    },
}

fn main() {
    let args = Args::parse();
    let output = format!("<t:{}>", Utc::now().timestamp());
    println!("{}", &output);
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(output.to_owned()).unwrap();
    println!("Copied to clipboard!");
}
