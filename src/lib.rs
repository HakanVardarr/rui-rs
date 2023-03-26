mod controller;

mod events;
pub mod keycode;
mod terminal;

pub use controller::Controller;
pub use crossterm::Result;
pub use terminal::Terminal;
