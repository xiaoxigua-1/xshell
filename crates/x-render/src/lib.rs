use std::fmt::Display;
use std::io::{Stdout, stdout, Write};
use x_protocol::crossterm::execute;
use x_input::Input;
use x_protocol::crossterm::cursor::MoveToColumn;
use x_protocol::crossterm::Result;
use x_protocol::InputState;
use x_protocol::crossterm::style::Print;
use x_protocol::crossterm::terminal::{Clear, ClearType};

pub struct Render {
    stdout: Stdout
}

impl Render {
    pub fn render(&mut self, input: &mut Input) -> Result<()> {
        self.user_input(&input.user_input)?;
        self.stdout.flush().unwrap();
        match input.state {
            InputState::NewLine => self.new_line(input)?,
            _ => {}
        }
        Ok(())
    }

    fn new_line(&mut self, input: &mut Input) -> Result<()> {
        input.user_input.clear();
        execute!(
            &self.stdout,
            MoveToColumn(0),
            Print("\n")
        )
    }

    fn user_input<T: Display>(&self, input: &T) -> Result<()> {
        execute!(
            &self.stdout,
            Clear(ClearType::CurrentLine),
            MoveToColumn(0),
            Print(input)
        )
    }
}

impl Default for Render {
    fn default() -> Self {
        Render { stdout: stdout() }
    }
}
