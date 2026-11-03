pub use crossterm::event::{KeyEvent, KeyCode, MouseEvent}; // TODO: Temporary. Eventually will use
                                                           // the custom backend

pub enum TerminalEvent {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub enum CrosstermEvent {
    Event(crossterm::event::Event),
    Tick,
}
impl CrosstermEvent {
    pub fn parse(&mut self) -> Option<TerminalEvent>{
        match self {
            CrosstermEvent::Event(event) => {
                match event {
                    crossterm::event::Event::FocusGained => None,
                    crossterm::event::Event::FocusLost => None,
                    crossterm::event::Event::Key(key_event) => Some(TerminalEvent::Keyboard(*key_event)),
                    crossterm::event::Event::Mouse(mouse_event) => Some(TerminalEvent::Mouse(*mouse_event)),
                    crossterm::event::Event::Paste(_) => None,
                    crossterm::event::Event::Resize(x, y) => Some(TerminalEvent::Resize(*x, *y)),
                }
            },
            CrosstermEvent::Tick => None,
        }
    }
}
