use x_protocol::ShellState;
use x_protocol::crossterm::event::{KeyCode, KeyModifiers, KeyEvent};
use x_protocol::crossterm::terminal;
use x_protocol::state::InputState;

#[derive(Debug, Clone)]
pub struct Input {
    pub user_input: String,
    pub cursor: u32,
    pub state: InputState,
}

impl Input {
    pub fn input(&mut self, code: &KeyEvent, state: &mut ShellState) {
        match code.modifiers {
            KeyModifiers::CONTROL => self.ctrl(code, state),
            KeyModifiers::ALT => self.alt(code),
            KeyModifiers::NONE => self.normal_input(code),
            _ => {}
        }
    }

    fn ctrl(&mut self, code: &KeyEvent, state: &mut ShellState) {
        if let KeyCode::Char(c) = code.code {
            self.user_input.push_str(format!("^{}", c.to_ascii_uppercase()).as_str());
            match c {
                'd' => {
                    state.is_exit = true;
                },
                'c' => self.state = InputState::NewLine,
                _ => {}
            }
        }
    }

    fn alt(&self, code: &KeyEvent) {

    }

    fn normal_input(&mut self, code: &KeyEvent) {
        self.state = InputState::NONE;
        match code.code {
            KeyCode::Char(c) => {
                self.cursor += 1;
                self.user_input.push(c);
            },
            KeyCode::Up => self.state = InputState::Up,
            KeyCode::Down => self.state = InputState::Down,
            KeyCode::Enter => self.state = InputState::NewLine,
            KeyCode::Backspace => {
                if self.cursor != 0 {
                    self.user_input.remove((self.cursor - 1) as usize);
                    self.cursor -= 1;
                }
            }
            _ => {}
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Input { user_input: String::new(), cursor: 0, state: InputState::NONE }
    }
}
