use rui_rs::components::Text;
use rui_rs::Ui;
use rui_rs::{Result, Terminal};

fn main() -> Result<()> {
    let mut terminal = Terminal::default(6, 2)?;
    let mut ui = Ui::new();

    ui.add_component(Box::new(Text::new(0, 0, "Hello,".to_string())));
    ui.add_component(Box::new(Text::new(0, 1, "World!".to_string())));
    terminal.add_ui(ui);

    terminal.run()
}
