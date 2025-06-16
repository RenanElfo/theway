mod format;
use format::format_text;
mod point;
use point::{
    get_point_index,
    point_from_index,
    point_number_from_index,
};

fn main() -> serde_json::Result<()> {
    let point_index = get_point_index();

    let point = point_from_index(point_index)?;

    println!("{}. {}", point_number_from_index(point_index), point.subject);
    for paragraph in point.paragraphs {
        println!("\n{}", format_text(paragraph.clone(), 50));
    }

    Ok(())
}
