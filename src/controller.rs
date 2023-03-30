use crate::events::Events;
use crossterm::event::KeyCode;
use crossterm::terminal::enable_raw_mode;
use crossterm::Result;

pub type Func = Box<dyn FnMut((u16, u16)) -> Result<(u16, u16)> + 'static>;

pub struct Controller {
    pub events: Events,
    pub row: u16,
    pub col: u16,
    pub maxcol: u16,
    pub maxrow: u16,
}

impl Controller {
    pub fn new(mut f: Func, maxcol: u16, maxrow: u16) -> Result<Self> {
        enable_raw_mode()?;
        f((0, 0))?;
        Ok(Self {
            events: Events { events: None },
            row: 0,
            col: 0,
            maxcol,
            maxrow,
        })
    }
    pub fn add_event(&mut self, code: KeyCode, b: bool, func: Func) {
        let events = self.events.events.get_or_insert_with(|| Vec::new());
        if events.iter().any(|(c, _, _)| *c == code) {
            return;
        } else {
            events.push((code, b, func));
        }
    }
}
