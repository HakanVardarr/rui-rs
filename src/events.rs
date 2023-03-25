use super::ui::Func;
use crossterm::event::KeyCode;

pub struct Events {
    pub events: Option<Vec<(KeyCode, bool, Func)>>,
}
