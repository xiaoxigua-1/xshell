use std::io::{BufReader, Stdin};

use x_input::Input;
use x_protocol::state::ShellState;
use x_render::Render;
use x_protocol::crossterm::Result;
use x_protocol::crossterm::event::{read, Event};
use x_protocol::crossterm::terminal::{enable_raw_mode, disable_raw_mode};


struct XShellEvent {
    stream: BufReader<Stdin>,
    state: ShellState,
    input: Input,
    render: Render,
}

impl XShellEvent {
    pub fn new(input: Stdin, state: ShellState) -> XShellEvent {
        XShellEvent { stream: BufReader::new(input), state, input: Input::default(), render: Render::default() }
    }

    pub fn listen_start(&mut self) -> Result<()> {
        enable_raw_mode()?;
        while !self.state.is_exit {
            match read()? {
                Event::Key(key) => self.input.input(&key, &mut self.state),
                _ => {}
            }

            self.render.render(&mut self.input)?;
        }

        self.exit()
    }

    fn exit(&self) -> Result<()> {
        disable_raw_mode()
    }
}

#[test]
fn test() {
    let mut x_shell_event = XShellEvent::new(std::io::stdin(), ShellState::default());
    x_shell_event.listen_start().unwrap();
}
