pub fn format_text(text: String, width: usize) -> String {
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

