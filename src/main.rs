mod parser;
use chrono::prelude::*;
use clap::{Parser, Subcommand};
use days::*;
use std::fs;
use std::time::Instant;

mod days;

const YEAR: usize = 2022;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(value_name = "DAY", help = "The day you want to run")]
        day: Option<String>,
    },
    GetInput {
        #[arg(value_name = "DAY", help = "The day you want to get the input for")]
        day: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { day } => match day {
            Some(day) => run_day(parse_day(day)),
            None => {
                let today = get_today();
                run_day(today);
            }
        },
        Commands::GetInput { day } => match day {
            Some(day) => download_input(parse_day(day)),
            None => {
                let today = get_today();
                download_input(today);
            }
        },
    }
}

fn get_today() -> usize {
    let now = Local::now();
    let now_day = now.day();
    if now.month() == 12 && (1..=25).contains(&now_day) {
        now_day.try_into().unwrap()
    } else {
        panic!("Today is not a part of Advent of Code.");
    }
}

fn parse_day(day: &str) -> usize {
    match day.parse() {
        Ok(i) => {
            if (i..=25).contains(&i) {
                i
            } else {
                panic!("{} is not a valid day. Only days 1-25 are allowed.", i)
            }
        }
        Err(_) => {
            panic!("{} is not a valid day. Please provide a number.", day)
        }
    }
}

// Panics if you provide a value outside the range of 1 to 25
fn run_day(day: usize) {
    let input_fp = &format!("inputs/day{:02}", day);
    match day {
        1 => day1::Day1::run_day(input_fp),
        2 => day2::Day2::run_day(input_fp),
        3 => day3::Day3::run_day(input_fp),
        7 => day7::Day7::run_day(input_fp),
        _ => panic!("Nope")
    }
}

fn download_input(day: usize) {
    let session = fs::read_to_string(".session").expect("Could not find .session file").replace(" ", "").replace("\n", "");
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("cookie", format!("session={};", session))
        .send()
        .unwrap();

    if response.status().is_success() {
        let mut text = response.text().unwrap();
        text.pop();
        let path = format!("inputs/day{:02}", day);
        fs::write(&path, text).unwrap();
        println!("Successfully downloaded input to {}", &path);
    } else {
        panic!(
            "Could not get input for day {}. Is your correct session cookie in your .session file?",
            day
        )
    }
}
