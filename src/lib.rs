pub mod keycodes;

use keycodes::KeyCode;

#[derive(Debug)]
pub struct PromptReader {
    result: String,
    cursor: usize,
    done: bool,
}

impl PromptReader {
    /// Create a new PromptReader initialized with an empty placeholder.
    /// The cursor will be at the start of the string
    pub fn new() -> Self {
        PromptReader {
            result: "".into(),
            cursor: 0,
            done: false,
        }
    }

    /// Create a PromptReader with the initial result ph and the cursor at position cursor_pos.
    /// If `cursor_pos` is None it will be set to the end of the string.
    pub fn new_with_placeholder(ph: &str, cursor_pos: Option<usize>) -> Self {
        PromptReader {
            result: ph.into(),
            cursor: cursor_pos.unwrap_or_else(|| ph.len()),
            done: false,
        }
    }

    /// Call this when you recieve a key event and want to pass it to the
    /// PromptReader. It will update its internal state and with based on the keycode input.
    pub fn next_key(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(c) => {
                if self.cursor >= self.result.len() {
                    self.result.push(c);
                } else {
                    self.result.insert(self.cursor, c);
                }
                self.cursor = self.cursor + 1;
            }
            KeyCode::Backspace => {
                if self.cursor != 0 {
                    self.result.remove(self.cursor - 1);
                    self.cursor = self.cursor - 1;
                }
            }
            KeyCode::Delete => {
                if self.cursor != self.result.len() {
                    self.result.remove(self.cursor);
                }
            }
            KeyCode::Left => {
                if self.cursor != 0 {
                    self.cursor = self.cursor - 1;
                }
            }
            KeyCode::Right => {
                if self.cursor >= self.result.len() - 1 {
                    self.cursor = self.cursor + 1;
                }
            }
            KeyCode::Enter => {
                self.done = true;
            }
            _ => {}
        }
    }

    /// Returns true after enter has been sent to the next_key function.
    pub fn done(&self) -> bool {
        self.done
    }

    /// Returns a reference to the resulting string
    pub fn result(&self) -> &str {
        &self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_string() {
        let mut pr = PromptReader::new();
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));

        assert_eq!(pr.result(), "Hej".to_string());
    }

    #[test]
    fn backspace() {
        let mut pr = PromptReader::new();
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));
        pr.next_key(KeyCode::Backspace);
        assert_eq!(pr.result(), "He".to_string());
    }

    #[test]
    fn left_and_backspace() {
        let mut pr = PromptReader::new();
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));
        pr.next_key(KeyCode::Left);
        pr.next_key(KeyCode::Backspace);
        assert_eq!(pr.result(), "Hj".to_string());
    }

    #[test]
    fn backspace_overflow() {
        let mut pr = PromptReader::new();
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));
        pr.next_key(KeyCode::Left);
        pr.next_key(KeyCode::Left);
        pr.next_key(KeyCode::Left);
        pr.next_key(KeyCode::Backspace);
        pr.next_key(KeyCode::Char('O'));

        assert_eq!(pr.result(), "OHej".to_string());
    }

    #[test]
    fn left_right_overflow() {
        let mut pr = PromptReader::new();
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));
        pr.next_key(KeyCode::Left);
        pr.next_key(KeyCode::Right);
        pr.next_key(KeyCode::Right);
        pr.next_key(KeyCode::Char('O'));

        assert_eq!(pr.result(), "HejO".to_string());
    }

    #[test]
    fn delete() {
        let mut pr = PromptReader::new();
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));
        pr.next_key(KeyCode::Left);
        pr.next_key(KeyCode::Delete);

        assert_eq!(pr.result(), "He".to_string());
    }

    #[test]
    fn delete_overflow() {
        let mut pr = PromptReader::new();
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));
        pr.next_key(KeyCode::Left);
        pr.next_key(KeyCode::Delete);
        pr.next_key(KeyCode::Delete);

        assert_eq!(pr.result(), "He".to_string());
    }

    #[test]
    fn placeholder() {
        let mut pr = PromptReader::new_with_placeholder("test", None);
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));

        assert_eq!(pr.result(), "testHej".to_string());
    }

    #[test]
    fn placeholder_with_cursor_pos() {
        let mut pr = PromptReader::new_with_placeholder("test", Some(2));
        pr.next_key(KeyCode::Char('H'));
        pr.next_key(KeyCode::Char('e'));
        pr.next_key(KeyCode::Char('j'));

        assert_eq!(pr.result(), "teHejst".to_string());
    }
}
