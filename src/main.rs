use std::fs;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::Result;

const NUMBER_OF_POINTS: i64 = 999;

#[derive(Serialize, Deserialize)]
struct Point {
    subject: String,
    point: String,
}

fn get_day_since_epoch() -> i64 {
    let current_date: NaiveDate = Local::now().date_naive();
    let epoch: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let day_difference = (current_date - epoch).num_days();
    return day_difference % NUMBER_OF_POINTS;
}

fn main() -> Result<()> {
    let day_difference = get_day_since_epoch();

    let file_path = std::env::args().nth(1).unwrap();
    let json_content = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");

    let points: Vec<Point> = serde_json::from_str(&json_content)?;
    let first_point: String = points[day_difference as usize].point.clone();

    println!("{first_point}");

    Ok(())
}
