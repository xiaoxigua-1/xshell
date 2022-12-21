use std::{path::PathBuf, collections::HashMap};
use x_util::{home_dir, whoami};

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

#[derive(Clone, Debug)]
pub struct ShellState {
    pub path: Option<PathBuf>,
    pub login: String,
    pub envs: HashMap<String, String>,
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
            is_exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.is_exit = true;
    }

    pub fn add_env(&mut self, key: String, value: String) {
        self.envs.insert(key, value);
    }
}

impl Default for ShellState {
    fn default() -> Self {
        ShellState {
            path: home_dir(),
            login: whoami().into(),
            envs: HashMap::new(),
            variables: HashMap::new(),
            is_exit: false,
        }
    }
}

#[test]
fn test() {
    let state = ShellState::default();
    println!("{:?}", state);
}
