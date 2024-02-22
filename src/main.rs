use chrono::{NaiveDateTime, Utc};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct DateEntry {
    hash: String,
    days_ago: i64,
}

fn main() {
    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    // Read the file and parse dates
    if let Err(err) = read_dates(file_path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn read_dates(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let now = Utc::now().naive_utc();

    let dates: Vec<DateEntry> = lines
        .filter_map(|line| {
            line.ok().and_then(|s| {
                let trimmed = s.trim();
                let mut split = trimmed.split(",");
                let hash = split.next()?;
                let date = split.next()?;
                let date = NaiveDateTime::parse_from_str(date, "%a %b %e %H:%M:%S %Y %z")
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                    .ok()?;
                let days_ago = now.signed_duration_since(date).num_days();
                Some(DateEntry {
                    hash: hash.to_string(),
                    days_ago,
                })
            })
        })
        .collect();

    dates.iter().for_each(|item| {
        println!("{},{}", item.days_ago, item.hash);
    });

    Ok(())
}
