use chrono::*;

use clap::Parser;

use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

const FORMATS: [char; 7] = ['t', 'T', 'd', 'D', 'f', 'F', 'R'];

/// A Discord Timestamp Generator
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(allow_negative_numbers = true)]
struct Cli {
    ///Sets the format of the timestamp.
    ///
    /// t - Short Time (16:20)
    ///
    /// T - Long Time (16:20:30)
    ///
    /// d - Short Date (20/04/2021)
    ///
    /// D - Long Date (20 April 2021),
    ///
    /// f - Short Date/Time (20 April 2021 16:20)
    ///
    /// F- Long Date/Time (Tuesday, 20 April 2021 16:20)
    ///
    /// R - Relative Time (	2 months ago)
    #[clap(short, long, default_value_t = 'f', verbatim_doc_comment, value_parser = valid_format)]
    format: char,
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
    let output = format!("<t:{}:{}>", ts, cli.format);
    println!("Timestamp:\n{}\n", &output);
    if cli.copy {
        let mut ctx = ClipboardContext::new().unwrap();
        let prev_contents = ctx.get_contents();
        match prev_contents {
            Ok(string) => println!("Previous clipboard contents: \n{}\n", string),
            Err(error) => {
                println!("Encountered an error while trying to output your previous clipboard contents: \n{}", error);
                println!(
                    "\x1b[93mThis often occurs from having an image saved in your clipboard!\x1b[0m\n"
                );
            }
        }

        ctx.set_contents(output.to_owned()).unwrap();
        println!("{} copied to clipboard!", output);
    }
}

fn is_leap(year: i32) -> bool {
    ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
}

fn valid_format(s: &str) -> Result<char, String> {
    let c = s.chars().nth(0).unwrap();
    for format in FORMATS {
        if c == format {
            return Ok(c);
        }
    }
    Err(format!(
        "Must contain one of the valid formats: {:?}",
        FORMATS
    ))
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
