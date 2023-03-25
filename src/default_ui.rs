use super::Ui;
use crossterm::cursor::{MoveTo, Show};
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::style::Color::Reset;
use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{
    disable_raw_mode, Clear, ClearType::FromCursorDown, ClearType::FromCursorUp,
};
use crossterm::Result;
use std::io::stdout;

pub fn default_ui(maxcol: u16, maxrow: u16) -> Result<Ui> {
    let mut ui = Ui::new(
        Box::new(|(_, _)| {
            execute!(stdout(), Clear(FromCursorUp), MoveTo(0, 0),)?;
            Ok((0, 0))
        }),
        maxrow,
        maxcol,
    )?;

    ui.add_event(
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

            Ok((ui.col, ui.row))
        }),
    );

    ui.add_event(
        KeyCode::Right,
        false,
        Box::new(move |(mut col, row)| {
            if col < ui.maxcol {
                col += 1;
            }
            execute!(stdout(), MoveTo(col, row))?;

            Ok((col, row))
        }),
    );

    ui.add_event(
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

    ui.add_event(
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

    ui.add_event(
        KeyCode::Down,
        false,
        Box::new(move |(col, mut row)| {
            if row < ui.maxrow {
                row += 1;
                execute!(stdout(), MoveTo(col, row))?;
            }

            Ok((col, row))
        }),
    );

    Ok(ui)
}
