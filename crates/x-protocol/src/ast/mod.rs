use crate::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Function {
        name: Token,
        parameters: Parameters,
        block: Block,
    },
    Command {
        name: Token,
        args: Vec<Expression>,
    },
    Call {
        name: Token,
    },
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub left: Token,
    pub variables: Vec<Token>,
    pub right: Token,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub left: Token,
    pub stmts: Vec<AST>,
    pub right: Token,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(Token),
    Ident(Token),
    Str(Token),
    Int(Token),
    Path(Token),
}
