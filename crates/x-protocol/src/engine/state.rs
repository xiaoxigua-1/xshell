use std::path::Path;

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
