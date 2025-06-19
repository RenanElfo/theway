pub fn format_text(text: String, width: usize) -> String {
    let italicized_text = convert_italic(&text);
    let mut out = String::new();
    let mut char_counter = 0;
    for word in italicized_text.split_whitespace() {
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

pub fn convert_italic(text: &str) -> String {
    return text.replace("<i>", "\x1b[3m").replace("</i>", "\x1b[23m");
}
