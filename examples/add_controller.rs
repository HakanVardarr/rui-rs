use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use crossterm::style::Color::Reset;
use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{
    disable_raw_mode, Clear, ClearType::FromCursorDown, ClearType::FromCursorUp,
};
use rui_rs::{keycode, Controller, Result, Terminal};
use std::io::stdout;
fn main() -> Result<()> {
    let mut terminal = Terminal::new(12, 50);
    let mut controller = Controller::new(
        Box::new(|(_, _)| {
            execute!(stdout(), Clear(FromCursorUp), MoveTo(0, 0),)?;

            Ok((0, 0))
        }),
        12,
        50,
    )?;
    controller.add_event(
        keycode::KeyCode::Char('q'),
        true,
        Box::new(|(_, _)| {
            execute!(
                stdout(),
                MoveTo(0, 0),
                SetForegroundColor(Reset),
                SetBackgroundColor(Reset),
                Clear(FromCursorDown),
                Show,
            )?;

            disable_raw_mode()?;

            Ok((0, 0))
        }),
    );

    terminal.add_controller(controller);
    terminal.run()
}
