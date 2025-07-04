use clap::Parser;

mod format; use format::format_text;
mod point; use point::Point;
mod book; use book::BOOKS;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Language to display the point in. Available locales:
    ///     es, bg, ca, cs, de, en, fi, fr, hr, it, ja, ko, lt, lv, hu, nl, pl,
    ///     pt-br, pt-pt, ro, ru, sk, sl, sv, zh-hans, zh-hant
    #[arg(short, long, default_value_t = String::from("en"))]
    locale: String,
    /// Max width for displaying the point.
    #[arg(short, long, default_value_t = 50)]
    width: usize,
    /// Display specified point number instead of pseudo-random one.
    #[arg(short, long)]
    point: Option<i32>,
}

fn main() {
    let args = Args::parse();
    let book_content = BOOKS
        .iter()
        .find(|&element| element.locale == args.locale)
        .unwrap()
        .content;
    let point = Point::new(book_content, args.point);

    println!("{}. {}", point.number, point.subject);
    for paragraph in point.paragraphs {
        println!("\n{}", format_text(paragraph.clone(), args.width));
    }
}
