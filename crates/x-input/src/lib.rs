use x_protocol::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use x_protocol::state::InputState;
use x_protocol::ShellState;

#[derive(Debug, Clone)]
pub struct Input {
    pub user_input: String,
    pub cursor: usize,
    pub state: InputState,
}

fn char_len(c: &char) -> usize {
    if c.is_ascii() {
        1
    } else {
        3
    }
}

impl Input {
    pub fn input(&mut self, code: &KeyEvent, state: &mut ShellState) {
        self.state = InputState::NONE;
        match code.modifiers {
            KeyModifiers::CONTROL => self.ctrl(code, state),
            _ => self.normal_input(code),
        }
    }

    pub fn clear(&mut self) {
        self.cursor = 0;
        self.user_input.clear();
    }

    fn ctrl(&mut self, code: &KeyEvent, state: &mut ShellState) {
        if let KeyCode::Char(c) = code.code {
            self.user_input
                .push_str(format!("^{}", c.to_ascii_uppercase()).as_str());
            match c {
                'd' => {
                    state.is_exit = true;
                }
                'c' => self.state = InputState::NewLine,
                _ => {}
            }
        }
    }

    fn normal_input(&mut self, code: &KeyEvent) {
        match code.code {
            KeyCode::Char(c) => {
                self.user_input.insert(self.cursor, c);
                self.cursor += char_len(&c);
            }
            KeyCode::Up => self.state = InputState::Up,
            KeyCode::Down => self.state = InputState::Down,
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Enter => self.state = InputState::Execute,
            KeyCode::Backspace => {
                self.left();
                if self.user_input.len() != 0 {
                    self.user_input.remove(self.cursor);
                }
            }
            _ => {}
        }
    }

    fn left(&mut self) {
        if self.cursor != 0 {
            if let Some(c) = &self.user_input[0..self.cursor].chars().last() {
                self.cursor -= char_len(c);
            }
        }
    }

    fn right(&mut self) {
        if self.cursor <= self.user_input.len() {
            if let Some(c) = &self.user_input[self.cursor..self.user_input.len()]
                .chars()
                .next()
            {
                self.cursor += char_len(c);
            }
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            user_input: String::new(),
            cursor: 0,
            state: InputState::NONE,
        }
    }
}
