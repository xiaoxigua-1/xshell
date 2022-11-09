use std::io::{BufReader, Stdin};

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
        enable_raw_mode()?;
        loop {
            match read()? {
                Event::Key(k) => {
                    match k.modifiers {
                        KeyModifiers::CONTROL => {
                            self.ctrl(&k.code)
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn ctrl(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Char(c) => {
                match c {
                    'd' | 'D' => std::process::exit(1),
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
