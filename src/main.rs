use std::fs;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

const LCG_MULTIPLIER: i64 = 445;
const LCG_INCREMENT: i64 = 713;
const NUMBER_OF_POINTS: i64 = 999;

#[derive(Clone, Serialize, Deserialize)]
struct Point {
    subject: String,
    paragraphs: Vec<String>,
}

fn format_text(text: String, width: usize) -> String {
    let mut out = String::new();
    let mut char_counter = 0;
    for word in text.split_whitespace() {
        if char_counter + word.len() > width {
            out.push('\n');
            char_counter = 0;
        } else {
            if char_counter != 0 { out.push(' ') }
        }
        char_counter += word.len() + 1;
        out.push_str(word);
    }

    return out;
}

fn get_point_index() -> i64 {
    let current_date: NaiveDate = Local::now().date_naive();
    let epoch: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let day_difference = (current_date - epoch).num_days();
    return day_difference % NUMBER_OF_POINTS;
}

fn point_from_index(index: i64) -> Result<Point, serde_json::Error> {
    let point_number = point_number_from_index(index);

    let file_path = std::env::args().nth(1).unwrap();
    let json_content = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");

    let points: Vec<Point> = serde_json::from_str(&json_content)?;
    let zero_indexed_point_number: usize = point_number - 1;
    let point: Point = points[zero_indexed_point_number].clone();

    return Ok(point);
}

fn point_number_from_index(index: i64) -> usize {
    let mut zero_indexed_point_number: i64 = 0;
    for _ in 0..=index {
        zero_indexed_point_number =
            (LCG_MULTIPLIER * zero_indexed_point_number + LCG_INCREMENT)
            % NUMBER_OF_POINTS;
    }

    return (zero_indexed_point_number + 1) as usize;
}

fn main() -> serde_json::Result<()> {
    let point_index = get_point_index();

    let point = point_from_index(point_index)?;

    println!("{}. {}", point_number_from_index(point_index), point.subject);
    for paragraph in point.paragraphs {
        println!("\n{}", format_text(paragraph.clone(), 50));
    }

    Ok(())
}
