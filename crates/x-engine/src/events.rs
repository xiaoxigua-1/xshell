use x_input::Input;
use x_protocol::crossterm::event::{read, Event};
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
            let output_str = repl(&mut input);
            let cursor_index = {
                let mut index = 0;

                input.user_input[input.cursor..input.user_input.len()]
                    .chars()
                    .for_each(|c| {
                        index += if c.is_ascii() { 1 } else { 2 };
                    });
                index + 1
            };
            render.clear_line()?;
            render.render(&input.state, output_str, cursor_index, &self.state)?;

            match read()? {
                Event::Key(key) => input.input(&key, &mut self.state),
                _ => {}
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
