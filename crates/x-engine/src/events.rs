use std::time::Duration;

use x_input::Input;
use x_protocol::crossterm::event::{read, Event, poll};
use x_protocol::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use x_protocol::crossterm::Result;
use x_protocol::state::ShellState;
use x_render::Render;

use crate::repl::repl;

pub struct XShellEvent {
    state: ShellState,
}

impl XShellEvent {
    pub fn new(state: ShellState) -> XShellEvent {
        XShellEvent { state }
    }

    pub fn listen_start(&mut self) -> Result<()> {
        let mut render = Render::default();
        let mut input = Input::default();

        enable_raw_mode()?;
        render.output_state(&self.state)?;
        while !self.state.is_exit {
            if poll(Duration::from_millis(100))? {
                match read()? {
                    Event::Key(key) => input.input(&key, &mut self.state),
                    _ => {}
                }
            
                repl(&mut render, &mut input, &mut self.state)?;
            }
        }

        self.exit()
    }

    fn exit(&self) -> Result<()> {
        disable_raw_mode()
    }
}

impl Default for XShellEvent {
    fn default() -> Self {
        XShellEvent::new(ShellState::default())
    }
}

#[test]
fn test() {
    let mut x_shell_event = XShellEvent::new(ShellState::default());
    x_shell_event.listen_start().unwrap();
}
