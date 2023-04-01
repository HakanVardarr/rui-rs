use rui_rs::{Result, Terminal};

fn main() -> Result<()> {
    let mut terminal = Terminal::default(50, 50)?;

    terminal.run()
}
