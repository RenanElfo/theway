use clap::Parser;

mod format; use format::format_text;
mod point; use point::Point;
mod book; use book::BOOKS;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("en"))]
    locale: String,
    #[arg(short, long, default_value_t = 50)]
    width: usize,
}

fn main() {
    let args = Args::parse();
    let book_content = BOOKS
        .iter()
        .find(|&element| element.locale == args.locale)
        .unwrap()
        .content;
    let point = Point::new(book_content);

    println!("{}. {}", point.number, point.subject);
    for paragraph in point.paragraphs {
        println!("\n{}", format_text(paragraph.clone(), args.width));
    }
}
