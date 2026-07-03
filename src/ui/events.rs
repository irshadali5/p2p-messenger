//! Event loop.

pub enum AppEvent {
    Tick,
    Key(crossterm::event::KeyEvent),
    Quit,
}
