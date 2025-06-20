use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

const LCG_MULTIPLIER: i32 = 445;
const LCG_INCREMENT: i32 = 713;
const NUMBER_OF_POINTS: i32 = 999;

#[derive(Clone, Serialize, Deserialize)]
struct PointContent {
    subject: String,
    paragraphs: Vec<String>,
}

pub struct Point {
    pub number: i32,
    pub subject: String,
    pub paragraphs: Vec<String>,
}

impl Point {
    pub fn new(book_content: &'static str, point_number: Option<i32>) -> Point {
        let point_index = get_point_index();
        let point_number = match point_number {
            Some(value) => value,
            None => point_number_from_index(point_index),
        };
        let content = content_from_index(point_index, book_content)
            .expect("Could not create point: no content");
        return Point {
            number: point_number,
            subject: content.subject,
            paragraphs: content.paragraphs,
        };
    }
}

fn get_point_index() -> i32 {
    let current_date: NaiveDate = Local::now().date_naive();
    let epoch: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let day_difference = (current_date - epoch).num_days() as i32;
    return day_difference % NUMBER_OF_POINTS;
}

fn point_number_from_index(index: i32) -> i32 {
    let mut zero_indexed_point_number: i32 = 0;
    for _ in 0..=index {
        zero_indexed_point_number =
            (LCG_MULTIPLIER * zero_indexed_point_number + LCG_INCREMENT)
            % NUMBER_OF_POINTS;
    }

    return zero_indexed_point_number + 1;
}

fn content_from_index(index: i32, book_content: &'static str)
        -> Result<PointContent, serde_json::Error> {
    let point_number = point_number_from_index(index);

    let points: Vec<PointContent> = serde_json::from_str(book_content)?;
    let zero_indexed_point_number = (point_number - 1) as usize;
    let point: PointContent = points[zero_indexed_point_number].clone();

    return Ok(point);
}

// fn filepath_from_locale(locale: String) -> String {
// }
