mod format;
use format::format_text;
mod point;
use point::{Point};

fn main() -> serde_json::Result<()> {
    let point = Point::new();

    println!("{}. {}", point.number, point.subject);
    for paragraph in point.paragraphs {
        println!("\n{}", format_text(paragraph.clone(), 50));
    }

    Ok(())
}
