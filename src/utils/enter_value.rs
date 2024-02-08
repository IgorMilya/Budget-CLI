use std::io;
use std::io::Write;
use termcolor::Color;
use crate::structures::text_style::TextStyle;

pub fn enter_value(comment: &str, error: &str) -> String {

    let text_color = TextStyle::new()
        .color(Color::Magenta)
        .underline();

    text_color.set_styles_for_text(comment);

    io::stdout().flush().unwrap();
    let mut written_name = String::new();
    io::stdin().read_line(&mut written_name).expect(error);
    written_name
}