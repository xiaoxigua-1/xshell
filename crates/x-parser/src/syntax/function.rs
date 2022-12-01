use x_protocol::ast::{Parameters, AST};
use x_protocol::crossterm::style::Stylize;
use x_protocol::Result;
use x_util::debug;

use crate::Parser;
use x_protocol::{Token, Tokens};

impl<'a> Parser<'a> {
    pub fn function_syntax(&mut self) -> Result<AST> {
        let (_, name) = self.eat_token_eq_default(
            |token| {
                if let Tokens::Ident(_) = token.ty {
                    true
                } else {
                    false
                }
            },
            "Missing function name",
        )?;
        debug!("Parse function name `{}`", name.ty.to_string());
        let parameters = self.parameters()?;
        Ok(AST::Function { name, parameters })
    }

    fn parameters(&mut self) -> Result<Parameters> {
        let (left_i, left) = self.eat_token_eq_default(
            |token| {
                if let Tokens::Symbol(symbol) = &token.ty {
                    symbol.eq(&'(')
                } else {
                    false
                }
            },
            "Missing left parentheses",
        )?;
        let mut variables: Vec<Token> = vec![];
        let right = loop {
            self.eat_whitespace()?;
            if let Some((_, right)) = self.lexer.next_if(|(_, token)| {
                let Ok(token) = token else {
                    return false;
                };
                token.ty.eq(&Tokens::Symbol(')'))
            }) {
                let right = right?;
                self.output_str(right.ty.default_highlighter());
                break right;
            }
            let Some((_, token)) = self.lexer.next() else {
                return Err(x_protocol::ShellErr::Unterminated(
                        left.span.clone(),
                        left_i,
                        "Missing right parentheses brackets.".into()
                ));
            };

            let token = token?;
            match token.ty {
                Tokens::Ident(_) => {
                    self.output_str(token.ty.default_highlighter());
                    variables.push(token)
                },
                Tokens::EOF => {
                    return Err(x_protocol::ShellErr::Unterminated(
                        left.span.clone(),
                        left_i,
                        "Missing right parentheses brackets.".into()
                    ));
                }
                _ => {
                    self.output_str(token.ty.default_highlighter());
                    return Err(x_protocol::ShellErr::Syntax(token.span, format!("{:?}", token.ty)))
                } 
            }
        };

        debug!(
            "Parse function parameter [{}]",
            variables
                .iter()
                .map(|v| { v.ty.to_string() })
                .collect::<Vec<String>>()
                .join(", ")
        );

        Ok(Parameters {
            left,
            variables,
            right,
        })
    }
}
