use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use chrono::NaiveDateTime;

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
    let now = chrono::offset::Utc::now().naive_utc();

    let dates: Vec<(String, i64)> = lines
        .filter_map(|line| {
            line.ok().and_then(|s| {
                let trimmed = s.trim();
                let mut split = trimmed.split(",");
                let hash = split.next().expect("could not find first element");
                let date = split.next().expect("could not find second element");
                let date = NaiveDateTime::parse_from_str(date, "%a %b %e %H:%M:%S %Y %z")
                    .expect("could not parse date");
                let days_ago = now.signed_duration_since(date).num_days();
                Some((hash.to_string(), days_ago))
            })
        })
        .collect();

    dates.iter().for_each(|item| {
        println!("{},{}", item.0, item.1);
    })
}

