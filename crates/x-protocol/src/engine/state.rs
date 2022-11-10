use std::path::Path;
use x_util::whoami;

pub struct ShellState<'a> {
    pub path: &'a Path,
    pub login: String,
    pub user_input: String,
}

impl<'a> ShellState<'a> {
    pub fn new(dir: &'a Path, login: String) -> Self {
        ShellState { path: dir, login, user_input: String::new() }
    }

    pub fn clear_input(&mut self) {
        self.user_input.clear();
    }
}

impl Default for ShellState<'_> {
    fn default() -> Self {
        ShellState { path: Path::new("~"), login: whoami().into(), user_input: String::new() }
    }
}
