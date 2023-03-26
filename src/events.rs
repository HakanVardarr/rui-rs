use super::controller::Func;
use crossterm::event::KeyCode;

pub struct Events {
    pub events: Option<Vec<(KeyCode, bool, Func)>>,
}
