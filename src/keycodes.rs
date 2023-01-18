#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    Char(char),
    Null,
    Esc,
}

impl From<char> for KeyCode {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}
