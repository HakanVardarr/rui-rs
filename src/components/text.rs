use super::Component;
use super::Result;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use std::io::stdout;

pub struct Text {
    pub col: u16,
    pub row: u16,
    pub text: String,
}

impl Text {
    pub fn new(col: u16, row: u16, text: String) -> Self {
        Self { col, row, text }
    }
}

impl Component for Text {
    fn draw(&self) -> Result<()> {
        execute!(stdout(), MoveTo(self.col, self.row), Print(&self.text),)
    }
}
