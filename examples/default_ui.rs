use rui_rs::{default_ui, Result};

fn main() -> Result<()> {
    let mut ui = default_ui(12, 50)?;

    ui.run()?;

    Ok(())
}
