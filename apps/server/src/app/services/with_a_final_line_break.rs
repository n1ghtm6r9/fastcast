pub fn with_a_final_line_break(mut text: String) -> String {
    if !text.ends_with('\n') {
        text.push('\n');
    }

    text
}
