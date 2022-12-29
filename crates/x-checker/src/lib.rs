use x_protocol::{ast::AST, ShellState};

pub struct Checker {
    asts: Vec<AST>,
    state: ShellState,
}

impl Checker {
    pub fn new(asts: Vec<AST>, state: ShellState) -> Self {
        Checker { asts, state }
    }
}
