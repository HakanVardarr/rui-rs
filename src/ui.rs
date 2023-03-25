use crate::events::Events;
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::enable_raw_mode;
use crossterm::Result;

pub type Func = Box<dyn FnMut((u16, u16)) -> Result<(u16, u16)> + 'static>;

pub struct Ui {
    pub events: Events,
    pub row: u16,
    pub col: u16,
    pub maxcol: u16,
    pub maxrow: u16,
}

impl Ui {
    pub fn new(mut f: Func, maxrow: u16, maxcol: u16) -> Result<Self> {
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
        if let Some(mut events) = self.events.events.take() {
            for (c, _, _) in events.iter() {
                if code == *c {
                    self.events.events = Some(events);
                    return;
                }
            }
            events.push((code, b, func));
            self.events.events = Some(events);
        } else {
            let mut events = Vec::new();
            events.push((code, b, func));
            self.events.events = Some(events);
        }
    }
    pub fn run(&mut self) -> Result<()> {
        if let Some(mut event) = self.events.events.take() {
            let mut col = 0;
            let mut row = 0;
            'outer: loop {
                match read()? {
                    Event::Key(key) => {
                        for t in event.iter_mut() {
                            if t.0 == key.code && key.kind == KeyEventKind::Press {
                                let (c, r) = t.2((col, row))?;
                                col = c;
                                row = r;

                                if t.1 == true {
                                    break 'outer;
                                }

                                continue;
                            } else {
                                continue;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    }
}
