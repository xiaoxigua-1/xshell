use crate::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Function {
        name: Token,
        parameters: Parameters 
    },
    Command {
        
    }
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub left: Token,
    pub variable: Vec<Token>,
    pub right: Token,
}
