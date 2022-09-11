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
    let mut dt = Utc::now();

    let mut count = cli.years;
    while count > 0 {
        if is_leap(dt.year()) {
            dt += Duration::seconds(31622400)
        } else {
            dt += Duration::seconds(31536000)
        }
        count -= 1;
    }

    dt += Duration::weeks(cli.weeks)
        + Duration::days(cli.days)
        + Duration::hours(cli.hours)
        + Duration::minutes(cli.min)
        + Duration::seconds(cli.sec);

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

fn is_leap(year: i32) -> bool {
    ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_leap_test() {
        assert_eq!(is_leap(2000), true);
        assert_eq!(is_leap(1900), false);
        assert_eq!(is_leap(2024), true);
        assert_eq!(is_leap(2028), true);
        assert_eq!(is_leap(2032), true);
    }
}
