use std::{fmt::Debug, path::PathBuf};

use crate::Result;
use crate::example::Example;

pub trait Command: Debug {
    fn get_name(&self) -> &str;

    fn get_example(&self) -> Vec<Example> {
        vec![]
    }

    fn get_usage(&self) -> &str;

    fn run(&self, args: Vec<String>) -> Result<()> {
        Ok(())
    }

    fn is_sub(&self) -> bool {
        false
    }

    fn is_builtin(&self) -> bool {
        false
    }
}

pub struct BuiltinCommand<'a> {
    name: &'a str,
    usage: &'a str,
    func: Box<dyn Fn(Vec<String>) -> Result<()>>
}

pub struct EnvCommand {
    name: String,
    path: PathBuf,
    usage: String,
    func: Box<dyn Fn(Vec<String>) -> Result<()>>
}

impl Command for BuiltinCommand<'static> {
    fn is_builtin(&self) -> bool {
        true
    }

    fn run(&self, _: Vec<String>) -> Result<()> {
        Ok(())
    }

    fn get_usage(&self) -> &str {
        self.usage
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

impl Command for EnvCommand {
    fn run(&self, args: Vec<String>) -> Result<()> {
        (self.func)(args)
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

impl Debug for BuiltinCommand<'static> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command name: {}\nUsage: {}", self.name, self.usage)
    }
}

impl<'a> BuiltinCommand<'a> {
    pub fn new(name: &'a str, usage: &'a str, func: Box<dyn Fn(Vec<String>) -> Result<()>>) -> Self
    {
        BuiltinCommand { name, usage, func }
    }
}

impl EnvCommand {
    pub fn edit_usage(&mut self, usage: String) {
        self.usage = usage;
    }
}
