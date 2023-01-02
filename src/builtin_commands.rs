use std::fmt::Debug;

use x_engine::ShellState;
use x_engine::Result;
use x_engine::Command;

#[derive(Clone)]
pub struct BuiltinCommand<'a, F> {
    name: &'a str,
    usage: &'a str,
    func: Box<F>
}

impl<F> Command for BuiltinCommand<'static, F>
where F: Fn(Vec<String>, &mut ShellState) -> Result<()> + Clone + 'static 
{
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_usage(&self) -> &str {
        self.usage
    }

    fn run(&self, state: &mut ShellState, args: Vec<String>) -> Result<()> {
        (self.func)(args, state)
    }

    fn is_builtin(&self) -> bool {
        true
    }
}

impl<F> Debug for BuiltinCommand<'static, F>
where F: Fn(Vec<String>, &mut ShellState) -> Result<()>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command name: {}\nUsage: {}", self.name, self.usage)
    }
}

impl<'a, F> BuiltinCommand<'a, F>
where F: Fn(Vec<String>, &mut ShellState) -> Result<()> + Clone + 'static
{
    pub fn new(name: &'a str, usage: &'a str, func: Box<F>) -> Self
    {
        BuiltinCommand { name, usage, func }
    }
}

macro_rules! create_command {
    ($commands: ident, $name: literal, $usage: literal, $func: expr) => {
        let command = BuiltinCommand::new($name, $usage, Box::new($func));
        $commands.push(Box::new(command));
    };
}

pub fn get_commands() -> Vec<Box<dyn Command>> {
    let mut commands: Vec<Box<dyn Command>> = vec![];
    
    create_command!(
        commands,
        "exit",
        "",
        |_, state: &mut ShellState| {
            state.exit();
            Ok(())
        }
    );
    commands
}

