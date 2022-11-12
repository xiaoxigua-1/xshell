use std::fmt::Display;
use std::io::{Stdout, stdout};
use x_protocol::crossterm::execute;
use x_protocol::crossterm::cursor::{MoveToColumn, MoveUp};
use x_protocol::crossterm::Result;
use x_protocol::{InputState, ShellState};
use x_protocol::crossterm::style::Print;
use x_protocol::crossterm::terminal::{Clear, ClearType};

pub struct Render {
    stdout: Stdout
}

impl Render {
    pub fn render(&mut self, state: &InputState, input: String, shell_state: &ShellState) -> Result<()> {
        self.output_state(shell_state)?;
        self.user_input(input)?;
        match state {
            InputState::NewLine | InputState::Execute => self.new_line(shell_state)?,
            _ => {}
        }
        Ok(())
    }

    fn new_line(&mut self, shell_state: &ShellState) -> Result<()> {
        execute!(
            &self.stdout,
            Print("\n"),
            MoveToColumn(0)
        )?;
        self.output_state(shell_state)
    }

    pub fn clear_line(&self) -> Result<()> {
        execute!(
            &self.stdout,
            Clear(ClearType::CurrentLine),
            MoveToColumn(0)
        )
    }

    fn user_input<T: Display>(&self, input: T) -> Result<()> {
        execute!(
            &self.stdout,
            Print(input)
        )
    }

    pub fn output_state(&self, state: &ShellState) -> Result<()> {
        execute!(
            &self.stdout,
            Print(format!("{}@{}: ", state.login, state.path.clone().unwrap().as_path().to_str().unwrap()))
        )
    }
}

impl Default for Render {
    fn default() -> Self {
        Render { stdout: stdout() }
    }
}
