use x_protocol::ast::AST;

pub struct Checker {
    asts: Vec<AST>,
}

impl Checker {
    pub fn new(asts: Vec<AST>) -> Self {
        Checker { asts }
    }
}
