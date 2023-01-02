use std::{fmt::Debug, path::PathBuf};

use crate::{Result, ShellState};
use crate::example::Example;

pub trait Command: Debug + CommandClone {
    fn get_name(&self) -> &str;

    fn get_example(&self) -> Vec<Example> {
        vec![]
    }

    fn get_usage(&self) -> &str;

    fn run(&self, _: &mut ShellState, _: Vec<String>) -> Result<()> {
        Ok(())
    }

    fn is_sub(&self) -> bool {
        false
    }

    fn is_builtin(&self) -> bool {
        false
    }
}

pub trait CommandClone {
   fn clone_box(&self) -> Box<dyn Command>; 
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone,
{
    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct EnvCommand {
    name: String,
    path: PathBuf,
    usage: String,
}

impl Command for EnvCommand {
    fn run(&self, _: &mut ShellState, args: Vec<String>) -> Result<()> {
        Ok(())
    }

    fn get_usage(&self) -> &str {
        ""
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Debug for EnvCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command name: {}\nPath: {:?}\nUsage: {}", self.name, self.path, self.usage)
    }
}

impl EnvCommand {
    pub fn edit_usage(&mut self, usage: String) {
        self.usage = usage;
    }
}
