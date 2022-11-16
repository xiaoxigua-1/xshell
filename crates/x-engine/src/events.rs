use x_input::Input;
use x_parser::parser;
use x_protocol::crossterm::event::{read, Event};
use x_protocol::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use x_protocol::crossterm::Result;
use x_protocol::state::ShellState;
use x_render::Render;

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
        while !self.state.is_exit {
            render.clear_line()?;
            let output_str = match parser(&mut input) {
                Ok(output) => output.string,
                Err(e) => format!("{:?}", e),
            };
            render.render(&input.state, output_str, input.user_input.len() - input.cursor + 1, &self.state)?;

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
