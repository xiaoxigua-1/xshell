use std::path::Path;

use x_protocol::{ast::AST, ShellState};
use x_protocol::{Result, ShellErr};

pub struct Checker<'a> {
    state: &'a ShellState,
}

impl<'a> Checker<'a> {
    pub fn new(state: &'a ShellState) -> Self {
        Checker { state }
    }

    pub fn check(&self, ast: &AST) -> Result<()> {
        match ast {
            AST::Command { name, args } => self.command(name.index.clone(), name.ty.to_string())?,
            _ => {}
        }
        Ok(())
    }

    fn command(&self, index: usize, name: String) -> Result<()> {
        if self.state.commands.iter().find(|command| { command.get_name() == name }).is_some() {
            Ok(())
        } else {
            Err(ShellErr::UnknownCommand(index, name))
        }
    }
}
