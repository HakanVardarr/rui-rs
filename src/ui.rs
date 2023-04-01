use std::io::stdout;

use crossterm::cursor::MoveTo;
use crossterm::execute;

use super::Component;
use super::Result;

pub struct Ui {
    pub components: Vec<Box<dyn Component>>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub fn draw(&self, col: u16, row: u16) -> Result<()> {
        for component in self.components.iter() {
            component.draw()?;
            execute!(stdout(), MoveTo(col, row))?;
        }

        Ok(())
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
}
