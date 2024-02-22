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
    read_dates(file_path);

}

fn read_dates(file_path: &str) {
    let file = File::open(file_path).expect("failed to open file");
    let lines = io::BufReader::new(file).lines();

    let dates: Vec<(String, i64)> = lines
        .filter_map(|line| {
            line.ok().and_then(|s| {
                let trimmed = s.trim();
                let mut split = trimmed.split(",");
                let hash = split.next().expect("could not find first element");
                let date = split.next().expect("could not find second element");
                let date = NaiveDateTime::parse_from_str(date, "%a %b %e %H:%M:%S %Y %z").expect("could not parse date");
                let now = chrono::offset::Utc::now().naive_utc();
                let days_ago = now.signed_duration_since(date).num_days();
                Some((hash.to_string(), days_ago))
            })
        })
        .collect();

}

fn days_difference(date: NaiveDateTime) -> i64 {
    let now = chrono::offset::Utc::now().naive_utc();
    let duration = now.signed_duration_since(date);
    duration.num_days()
}

fn days_difference_with_hash(date: NaiveDateTime, hash: &str) -> (&str, i64) {
    let now = chrono::offset::Utc::now().naive_utc();
    let duration = now.signed_duration_since(date);
    (hash, duration.num_days())
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
