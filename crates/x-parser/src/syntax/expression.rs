use x_protocol::ast::Expression;
use x_protocol::{Result, Tokens};

use crate::Parser;

impl Parser<'_> {
    pub(crate) fn expressions(&mut self) -> Result<Expression> {
        use x_protocol::Tokens::*;
        let Some((_, token)) = self.lexer.next() else {
            return Err(x_protocol::ShellErr::EOF)
        };
        let token = token?;
        self.output.push(token.ty.default_highlighter());
        Ok(match token.ty {
            Ident(_) => Expression::Ident(token),
            Str(_) => Expression::Str(token),
            Int(_) => Expression::Int(token),
            Path(_) => Expression::Path(token),
            Symbol(c) if c.eq(&'$') => {
                let (_, name) = self.eat_token_eq_default(
                    |token| {
                        if let Tokens::Ident(_) = token.ty {
                            true
                        } else {
                            false
                        }
                    },
                    "Variable name error",
                )?;
                Expression::Variable(name)
            }
            Symbol(_) => Expression::Symbol(token),
            _ => {
                return Err(x_protocol::ShellErr::Syntax(
                    token.span,
                    "Not a expression".into(),
                ))
            }
        })
    }
}
