use std::ops::Range;
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
        if let Some(path) = self.state.envs.get("PATH") {
            let paths = path.split(":").map(|path| { path.to_string() }).collect::<Vec<String>>();
            for path in paths {
                if Path::new(&path).exists() {
                    return Ok(())
                }
            }
        }

        Err(ShellErr::UnknownCommand(index, name))
    }
}
