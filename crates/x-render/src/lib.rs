use std::fmt::Display;
use std::io::{stdout, Stdout};
use x_protocol::crossterm::cursor::{MoveLeft, MoveToColumn, RestorePosition, SavePosition};
use x_protocol::crossterm::execute;
use x_protocol::crossterm::style::{Print, Stylize};
use x_protocol::crossterm::terminal::{Clear, ClearType};
use x_protocol::crossterm::Result;
use x_protocol::ShellState;

pub struct Render {
    stdout: Stdout,
}

impl Render {
    pub fn new_line(&mut self, shell_state: &ShellState) -> Result<()> {
        execute!(&self.stdout, Print("\n"), MoveToColumn(0))?;
        self.output_state(shell_state)
    }

    pub fn clear_line(&self) -> Result<()> {
        execute!(
            &self.stdout,
            RestorePosition,
            Clear(ClearType::FromCursorDown)
        )
    }

    pub fn render<T: Display>(&self, input: T, cursor: usize) -> Result<()> {
        execute!(&self.stdout, Print(input), MoveLeft(cursor as u16))
    }

    pub fn output_state(&self, state: &ShellState) -> Result<()> {
        execute!(
            &self.stdout,
            Print(format!(
                "{}@{}: ",
                state.login.clone().green(),
                state
                    .path
                    .clone()
                    .unwrap()
                    .as_path()
                    .to_str()
                    .unwrap()
                    .blue()
            )),
            SavePosition
        )
    }
}

impl Default for Render {
    fn default() -> Self {
        Render { stdout: stdout() }
    }
}
