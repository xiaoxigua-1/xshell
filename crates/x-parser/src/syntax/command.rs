use x_protocol::ast::AST;
use x_protocol::{Result, Token, Tokens};

use crate::Parser;

impl Parser<'_> {
    pub fn command(&mut self, command: Token) -> Result<AST> {
        let name = match command.ty {
            Tokens::Str(s) => s[1..s.len() - 1].to_string(),
            ty @ _ => ty.to_string(),
        };
        Ok(AST::Command { name })
    }

    fn and(&mut self) {}

    fn or(&mut self) {}

    fn pipeline(&mut self) {}
}
