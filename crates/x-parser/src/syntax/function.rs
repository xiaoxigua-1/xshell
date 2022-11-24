use x_protocol::ast::{AST, Parameters};
use x_protocol::Result;

use crate::Parser;
use x_protocol::{Tokens, Token};

impl<'a> Parser<'a> {
    pub fn function_syntax(&mut self) -> Result<AST> {
        let name = self.eat_token_eq(|token| { if let Tokens::Ident(_) = token.ty { true } else { false } }, "Missing function name")?;
        println!("{:?}", name);
        let parameters = self.parameters()?; 
        Ok(AST::Function { 
            name,
            parameters
        })
    }

    fn parameters(&mut self) -> Result<Parameters> {
        let left = self.eat_token_eq(|token| {
            if let Tokens::Symbol(symbol) = &token.ty {
                symbol.eq(&'(')
            } else { false }
        }, "Missing left parentheses")?;
        let mut variable: Vec<Token> = vec![];
        let right = loop {
            if let Some(right) = self.lexer.next_if(|token| {
                match token {
                    Ok(token) => {
                        if let Tokens::Symbol(symbol) = &token.ty {
                            symbol.eq(&')')
                        } else {
                           false 
                        }
                    }
                    Err(_) => false,
                }
            }) {
                break right?;
            }

            variable.push(self.eat_token_eq(|token| {
                if let Tokens::Ident(_) = &token.ty {
                    true
                } else {
                    false
                }
            }, "Missing parameter")?);
        };

        Ok(Parameters {
            left,
            variable,
            right,
        })
    }
}
