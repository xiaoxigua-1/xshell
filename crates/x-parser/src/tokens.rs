use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Ident(String),
    Str(String),
    Int(String),
    Space(char),
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: Tokens,
    pub span: Range<usize>
}

impl Token {
    pub fn new(ty: Tokens, span: Range<usize>) -> Self {
        Token { ty, span }
    }
}
