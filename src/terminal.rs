use super::ui::Ui;
use super::{Controller, Result};
use crossterm::{
    cursor::{MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color::Reset, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, Clear, ClearType::FromCursorDown, ClearType::FromCursorUp},
};
use std::io::stdout;

pub struct Terminal {
    pub controller: Option<Controller>,
    pub ui: Option<Ui>,
    pub col: u16,
    pub row: u16,
}

impl Terminal {
    pub fn new(col: u16, row: u16) -> Self {
        Self {
            controller: None,
            ui: None,
            col,
            row,
        }
    }
    pub fn default(maxcol: u16, maxrow: u16) -> Result<Self> {
        let mut controller = Controller::new(
            Box::new(|(_, _)| {
                execute!(stdout(), Clear(FromCursorUp), MoveTo(0, 0),)?;
                Ok((0, 0))
            }),
            maxcol - 1,
            maxrow - 1,
        )?;

        controller.add_event(
            KeyCode::Esc,
            true,
            Box::new(move |(_, _)| {
                execute!(
                    stdout(),
                    MoveTo(0, 0),
                    SetForegroundColor(Reset),
                    SetBackgroundColor(Reset),
                    Clear(FromCursorDown),
                    Show,
                )?;
                disable_raw_mode()?;

                Ok((controller.col, controller.row))
            }),
        );

        controller.add_event(
            KeyCode::Right,
            false,
            Box::new(move |(mut col, row)| {
                if col < controller.maxcol {
                    col += 1;
                }
                execute!(stdout(), MoveTo(col, row))?;

                Ok((col, row))
            }),
        );

        controller.add_event(
            KeyCode::Left,
            false,
            Box::new(move |(mut col, row)| {
                if col > 0 {
                    col -= 1;
                    execute!(stdout(), MoveTo(col, row))?;
                }

                Ok((col, row))
            }),
        );

        controller.add_event(
            KeyCode::Up,
            false,
            Box::new(move |(col, mut row)| {
                if row > 0 {
                    row -= 1;
                    execute!(stdout(), MoveTo(col, row))?;
                }

                Ok((col, row))
            }),
        );

        controller.add_event(
            KeyCode::Down,
            false,
            Box::new(move |(col, mut row)| {
                if row < controller.maxrow {
                    row += 1;
                    execute!(stdout(), MoveTo(col, row))?;
                }

                Ok((col, row))
            }),
        );

        Ok(Self {
            controller: Some(controller),
            ui: None,
            col: maxcol,
            row: maxrow,
        })
    }
    pub fn add_controller(&mut self, c: Controller) {
        self.controller = Some(c);
    }
    pub fn add_ui(&mut self, ui: Ui) {
        self.ui = Some(ui);
    }
    pub fn run(&mut self) -> Result<()> {
        if let Some(mut c) = self.controller.take() {
            let mut col = 0;
            let mut row = 0;

            'outer: loop {
                self.ui.as_ref().unwrap().draw(col, row)?;
                match read()? {
                    Event::Key(key) => {
                        for a in c.events.events.iter_mut() {
                            for t in a.iter_mut() {
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
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    }
}
