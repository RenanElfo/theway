use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Point {
    subject: String,
    point: String,
}

fn main() -> Result<()> {
    let file_path = std::env::args().nth(1).unwrap();
    let json_content = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");

    let points: Vec<Point> = serde_json::from_str(&json_content)?;
    let first_point: String = points[0].point.clone();

    println!("{first_point}");

    Ok(())
}
