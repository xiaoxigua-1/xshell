use std::path::Path;
use x_util::whoami;

pub struct ShellState<'a> {
    path: &'a Path,
    login: String,
    user_input: String,
}

impl ShellState<'_> {
    pub fn clear_input(&mut self) {
        self.user_input.clear();
    }
}

impl Default for ShellState<'_> {
    fn default() -> Self {
        ShellState { path: Path::new("~"), login: whoami().into(), user_input: String::new() }
    }
}
