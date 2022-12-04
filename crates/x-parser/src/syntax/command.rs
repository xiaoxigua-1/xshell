use x_protocol::ast::AST;
use x_protocol::Tokens;
use x_protocol::{Result, Token};

use crate::Parser;

impl Parser<'_> {
    pub fn command(&mut self, name: Token) -> Result<AST> {
        let mut args: Vec<x_protocol::ast::Expression> = vec![];
        loop {
            self.eat_whitespace()?;
            if let Some((_, semicolon)) = self.lexer.next_if(|(_, token)| {
                let Ok(token) = token else {
                    return false;
                };
                token.ty.eq(&Tokens::Symbol(';'))
                    || token.ty.eq(&Tokens::NewLine)
                    || token.ty.eq(&Tokens::EOF)
            }) {
                self.output_str(semicolon?.ty.default_highlighter());
                break;
            }
            args.push(self.expressions()?)
        }
        Ok(AST::Command { name, args })
    }

    fn and(&mut self) {}

    fn or(&mut self) {}

    fn pipeline(&mut self) {}
}
