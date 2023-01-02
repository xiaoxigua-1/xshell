use std::{collections::HashMap, path::PathBuf, fs};
use x_util::{home_dir, whoami};

use crate::command::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum InputState {
    NewLine,
    Execute,
    Up,
    Down,
    Left,
    Right,
    NONE,
}

#[derive(Debug)]
pub struct ShellState {
    pub path: Option<PathBuf>,
    pub login: String,
    pub envs: HashMap<String, String>,
    pub commands: Vec<Box<dyn Command>>,
    pub variables: HashMap<String, String>,
    pub is_exit: bool,
}

impl ShellState {
    pub fn new(dir: PathBuf, login: String) -> Self {
        ShellState {
            path: Some(dir),
            login,
            envs: HashMap::new(),
            variables: HashMap::new(),
            commands: vec![],
            is_exit: false,
        }
    }

    pub fn init_commands(&mut self, commands: Vec<Box<dyn Command>>) {
        for command in commands {
            self.commands.push(command);
        }
    }

    pub fn exit(&mut self) {
        self.is_exit = true;
    }

    pub fn add_env(&mut self, key: String, value: String) {
        self.envs.insert(key, value);
    }

    pub fn updata(&mut self) {
        if let Some(path) = self.envs.get("PATH") {
            self.commands.clear();
            path.split(":").for_each(|path| {
                if let Ok(dir) = fs::read_dir(path) {
                    for file in dir {
                        
                    }
                }
                if self.commands.iter().find(|command| { command.get_name() == path }).is_none() {
                    
                };
            })
        }
    }
}

impl Default for ShellState {
    fn default() -> Self {
        ShellState {
            path: home_dir(),
            login: whoami().into(),
            envs: HashMap::new(),
            variables: HashMap::new(),
            commands: vec![],
            is_exit: false,
        }
    }
}

#[test]
fn test() {
    let state = ShellState::default();
    println!("{:?}", state);
}
