use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

pub struct TextStyle {
    pub color: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub bg: Option<Color>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            bold: false,
            italic: false,
            underline: false,
            bg: None,
        }
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn set_styles_for_text(&self, text: &str) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        let mut initial_color = ColorSpec::new();

        initial_color
            .set_fg(self.color)
            .set_bold(self.bold)
            .set_italic(self.italic)
            .set_underline(self.underline)
            .set_bg(self.bg);

        let _ = stdout.set_color(&initial_color);

        write!(&mut stdout, "{}", text).expect("Failed to write to stdout");

        let _ = stdout.reset();
    }
}