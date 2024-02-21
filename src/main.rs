use chrono::{Duration, NaiveDateTime};
use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Read the file and parse dates
    let dates: Vec<NaiveDateTime> = read_dates(file_path).expect("Error reading dates from file");

    // Calculate the difference in days
    let days_ago: Vec<i64> = dates.iter().map(|date| days_difference(*date)).collect();

    // Calculate and print the median
    let median_days = calculate_median(&days_ago);
    println!("Median days ago: {}", median_days);
}

fn read_dates(file_path: &str) -> io::Result<Vec<NaiveDateTime>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();

    let dates: Vec<NaiveDateTime> = lines
        .filter_map(|line| {
            line.ok().and_then(|s| {
                let trimmed = s.trim();
                NaiveDateTime::parse_from_str(trimmed, "%a %b %e %H:%M:%S %Y %z").ok()
            })
        })
        .collect();

    Ok(dates)
}

fn days_difference(date: NaiveDateTime) -> i64 {
    let now = chrono::offset::Utc::now().naive_utc();
    let duration = now.signed_duration_since(date);
    duration.num_days()
}

fn calculate_median(values: &Vec<i64>) -> i64 {
    let mut sorted_values = values.clone();
    sorted_values.sort();
    let n = sorted_values.len();
    println!("len: {}", n);

    if n % 2 == 1 {
        sorted_values[n / 2]
    } else {
        let mid = n / 2;
        (sorted_values[mid - 1] + sorted_values[mid]) / 2
    }
}
