use crate::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Function { name: Token, parameters: Parameters },
    Command { name: String },
    Call { name: Token },
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub left: Token,
    pub variables: Vec<Token>,
    pub right: Token,
}
