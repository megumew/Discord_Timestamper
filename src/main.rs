use chrono::*;

use clap::{Args, Parser, Subcommand};

use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

/// A Discord Timestamp Generator
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(allow_negative_numbers = true)]
struct Cli {
    ///Sets if you would like the output to be copied to clipboard
    #[clap(short, long)]
    copy: bool,
    ///Sets if you want years to be leap year accurate. Use when years is greater than 1
    #[clap(short, long)]
    leap: bool,
    ///Amount of seconds from now you would like the timestamp to represent
    #[clap(short, long, default_value_t = 0)]
    sec: i64,
    ///Amount of minutes from now you would like the timestamp to represent
    #[clap(short, long, default_value_t = 0)]
    min: i64,
    ///Amount of hours from now you would like the timestamp to represent
    #[clap(short, long, default_value_t = 0)]
    hours: i64,
    ///Amount of days from now you would like the timestamp to represent
    #[clap(short, long, default_value_t = 0)]
    days: i64,
    ///Amount of years from now you would like the timestamp to represent
    #[clap(short, long, default_value_t = 0)]
    weeks: i64,
    ///Amount of years from now you would like the timestamp to represent
    #[clap(short, long, default_value_t = 0)]
    years: i64,
}

fn main() {
    let cli = Cli::parse();
    let mut dt = Utc::now()
        + Duration::weeks(cli.weeks)
        + Duration::days(cli.days)
        + Duration::hours(cli.hours)
        + Duration::minutes(cli.min)
        + Duration::seconds(cli.sec);
    if cli.leap {
        dt += Duration::seconds(31557600 * cli.years)
    } else {
        dt += Duration::days(365 * cli.years);
    }
    let ts = dt.timestamp();
    let output = format!("<t:{}>", ts);
    println!("Timestamp:\n{}\n", &output);
    if cli.copy {
        let mut ctx = ClipboardContext::new().unwrap();
        println!(
            "Previous clipboard contents: \n{}\n",
            ctx.get_contents().unwrap()
        );
        ctx.set_contents(output.to_owned()).unwrap();
        println!("{} copied to clipboard!", output);
    }
}
