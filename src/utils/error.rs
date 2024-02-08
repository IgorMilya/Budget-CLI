use termcolor::Color;
use crate::structures::text_style::TextStyle;

pub fn error(text: &str){
    let text_color = TextStyle::new()
        .color(Color::Red)
        .bold();
    text_color.set_styles_for_text(text);
}

pub fn warning(text: &str){
    let text_color = TextStyle::new()
        .color(Color::Yellow)
        .bold();
    text_color.set_styles_for_text(text);
}