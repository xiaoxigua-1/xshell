use std::io::{BufReader, Stdin};

use x_input::Input;
use x_parser::parser;
use x_protocol::state::ShellState;
use x_render::Render;
use x_protocol::crossterm::Result;
use x_protocol::crossterm::event::{read, Event};
use x_protocol::crossterm::terminal::{enable_raw_mode, disable_raw_mode};


struct XShellEvent {
    stream: BufReader<Stdin>,
    state: ShellState,
}

impl XShellEvent {
    pub fn new(input: Stdin, state: ShellState) -> XShellEvent {
        XShellEvent { stream: BufReader::new(input), state }
    }

    pub fn listen_start(&mut self) -> Result<()> {
        let mut render = Render::default();
        let mut input = Input::default();

        enable_raw_mode()?;
        while !self.state.is_exit {
            render.clear_line()?;            
            let output_str = match parser(&mut input) {
                Ok(output) => output.string,
                Err(e) => format!("{:?}", e)
            };
            render.render(&input.state, output_str, &self.state)?;
            
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

#[test]
fn test() {
    let mut x_shell_event = XShellEvent::new(std::io::stdin(), ShellState::default());
    x_shell_event.listen_start().unwrap();
}
