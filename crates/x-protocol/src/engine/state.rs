use std::path::PathBuf;
use x_util::{whoami, home_dir};

#[derive(Clone, Debug)]
pub struct ShellState {
    pub path: Option<PathBuf>,
    pub login: String,
    pub user_input: String,
}

impl ShellState {
    pub fn new(dir: PathBuf, login: String) -> Self {
        ShellState { path: Some(dir), login, user_input: String::new() }
    }

    pub fn clear_input(&mut self) {
        self.user_input.clear();
    }
}

impl Default for ShellState {
    fn default() -> Self {
        ShellState { path: home_dir(), login: whoami().into(), user_input: String::new() }
    }
}

#[test]
fn test() {
    let state = ShellState::default();
    println!("{:?}", state);
}
