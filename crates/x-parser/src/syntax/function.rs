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

    pub fn call_function(&mut self, name: Token) -> Result<AST> {
        self.eat_token_eq_default(|token| token.ty.eq(&Tokens::Symbol('(')), "")?;
        todo!("parse args");
        Ok(AST::Call { name })
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
            if let Some((_, right)) = self.lexer.peek() {
                if let Ok(right) = right {
                    if let Tokens::Symbol(')') = right.ty {
                        let (_, right) = self.lexer.next().unwrap();
                        let right = right?;
                        self.output_str(right.ty.default_highlighter());
                        break right;
                    }
                }
            }
            let (_, variable) = self.eat_token_eq(
                |token| {
                    if let Tokens::Ident(_) = &token.ty {
                        true
                    } else {
                        false
                    }
                },
                |_, i| {
                    x_protocol::ShellErr::Unterminated(
                        left.span.clone(),
                        left_i,
                        "Missing right parentheses brackets.".into(),
                    )
                },
                Some(|s: String| s.blue()),
            )?;
            variables.push(variable);
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
