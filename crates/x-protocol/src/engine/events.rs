use std::io::{BufReader, Stdin, stdout};

use crossterm::cursor;
use crossterm::{style::Print, terminal, execute};
use crossterm::event::{read, Event, KeyModifiers, KeyCode};
use crossterm::terminal::enable_raw_mode;

use super::state::ShellState;

struct XShellEvent<'a> {
    stream: BufReader<Stdin>,
    state: ShellState<'a>
}

impl<'a> XShellEvent<'a> {
    pub fn new(input: Stdin, state: ShellState<'a>) -> XShellEvent<'a> {
        XShellEvent { stream: BufReader::new(input), state }
    }

    pub fn listen_start(&mut self) -> crossterm::Result<()> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        loop {
            match read()? {
                Event::Key(k) => {
                    match k.modifiers {
                        KeyModifiers::CONTROL => {
                            self.ctrl(&k.code);
                        }
                        KeyModifiers::NONE => match k.code {
                            KeyCode::Char(c) => self.state.user_input.push(c),
                            _ => {}
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;

            execute!(stdout, cursor::MoveToColumn(0) , Print(self.state.user_input.clone()))?;
        }
    }

    fn ctrl(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Char(c) => {
                match c {
                    'd' | 'D' => std::process::exit(0),
                    'c' => {
                        self.state.clear_input()
                    },
                    _ => {} 
                }
            }
            _ => {}
        }
    } 
}

#[test]
fn test() {
    let mut x_shell_event = XShellEvent::new(std::io::stdin(), ShellState::default());
    x_shell_event.listen_start().unwrap();
}
