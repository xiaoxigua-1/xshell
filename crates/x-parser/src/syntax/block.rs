use x_protocol::ast::{Block, AST};
use x_protocol::{Result, Tokens};

use crate::Parser;

impl Parser<'_> {
    pub fn pase_block(&mut self) -> Result<Block> {
        let mut stmts: Vec<AST> = vec![];
        let (left_i, left) = self.eat_token_eq_default(
            |token| token.ty.eq(&Tokens::Symbol('{')),
            "Missing left bracket.",
        )?;

        let right = loop {
            self.eat_whitespace()?;
            // right bracket
            if let Some((_, right)) = self.lexer.next_if(|(_, token)| {
                let Ok(token) = token else {
                    return false;
                };

                token.ty.eq(&Tokens::Symbol('}'))
            }) {
                let right = right?;
                self.output_str(right.ty.default_highlighter());
                break right;
            }
            let Some(ast) = self.parse()? else {
                return Err(x_protocol::ShellErr::Unterminated(left.span.clone(), left_i, "Missing right brackets.".into()))
            };
            stmts.push(ast);
        };

        Ok(Block { left, stmts, right })
    }
}
