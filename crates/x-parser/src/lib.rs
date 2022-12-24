mod lexer;
mod syntax;

use std::iter::Enumerate;
use std::{iter::Peekable, ops::Range};

pub use lexer::Lexer;
use x_protocol::ast::AST;
use x_protocol::crossterm::style::Stylize;
use x_protocol::shell_err::Result;
use x_protocol::ShellErr;
use x_protocol::{crossterm::style::StyledContent, Kwd, Token, Tokens};

pub struct Parser<'a> {
    lexer: Peekable<Enumerate<Lexer<'a>>>,
    pub output: Vec<StyledContent<String>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.enumerate().peekable(),
            output: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<Option<AST>> {
        Ok(if let Some((_, token)) = self.lexer.next() {
            let token = token?;
            self.output_str(token.ty.default_highlighter());
            let token = match &token.ty {
                Tokens::Keyword(k) => self.builtin(&k)?,
                Tokens::EOF => return Ok(None),
                _ => self.command(token)?,
            };
            if let Some((_, t)) = self.lexer.next_if(|(_, t)| {
                if let Ok(t) = t {
                    t.eq(Tokens::Symbol(';'))
                } else {
                    false
                }
            }) {
                self.output_str(t?.ty.default_highlighter());
            };
            Some(token)
        } else {
            None
        })
    }

    fn builtin(&mut self, kwd: &Kwd) -> Result<AST> {
        Ok(match kwd {
            Kwd::Function => self.function_syntax()?,
        })
    }

    fn output_str(&mut self, s: StyledContent<String>) {
        self.output.push(s);
    }

    fn eat_token_eq<F, E, S>(
        &mut self,
        eq_func: F,
        err: E,
        custom_style: Option<S>,
    ) -> Result<(usize, Token)>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce(Range<usize>, usize) -> ShellErr,
        S: FnOnce(String) -> StyledContent<String>,
    {
        self.eat_whitespace()?;
        let token = self.lexer.next();
        if let Some((i, token)) = token {
            let token = token?;
            if eq_func(&token) {
                let output = if let Some(highlighter) = custom_style {
                    token.ty.highlighter(highlighter)
                } else {
                    token.ty.default_highlighter()
                };
                self.output_str(output);
                Ok((i, token))
            } else {
                Err(err(token.span, i))
            }
        } else {
            Err(ShellErr::EOF)
        }
    }

    fn eat_token_eq_default<F>(&mut self, eq_func: F, err_message: &str) -> Result<(usize, Token)>
    where
        F: FnOnce(&Token) -> bool,
    {
        self.eat_token_eq(
            eq_func,
            |span, _| ShellErr::Syntax(span, err_message.into()),
            None::<Box<dyn FnOnce(String) -> StyledContent<String>>>,
        )
    }

    fn eat_token_eq_custom_err<F, E>(&mut self, eq_func: F, err: E) -> Result<(usize, Token)>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce(Range<usize>, usize) -> ShellErr,
    {
        self.eat_token_eq(
            eq_func,
            err,
            None::<Box<dyn FnOnce(String) -> StyledContent<String>>>,
        )
    }

    fn eat_token_eq_custom_color<F, S>(
        &mut self,
        eq_func: F,
        err_message: &str,
        custom_style: S,
    ) -> Result<(usize, Token)>
    where
        F: FnOnce(&Token) -> bool,
        S: FnOnce(String) -> StyledContent<String>,
    {
        self.eat_token_eq(
            eq_func,
            |span, _| ShellErr::Syntax(span, err_message.into()),
            Some(custom_style),
        )
    }

    fn eat_whitespace(&mut self) -> Result<()> {
        loop {
            if let Some((_, t)) = self.lexer.peek() {
                match t {
                    Ok(t) => match t.ty {
                        Tokens::Space(c) => {
                            self.output.push(c.to_string().stylize());
                            self.lexer.next()
                        }
                        _ => break Ok(()),
                    },
                    Err(_) => break Ok(()),
                };
            } else {
                break Ok(());
            };
        }
    }

    pub fn eat_remaining_token(&mut self) -> Result<StyledContent<String>> {
        if let Some((_, token)) = self.lexer.next() {
            let token = token?;
            Ok(token.ty.default_highlighter())
        } else {
            Err(ShellErr::EOF)
        }
    }
}

#[cfg(test)]
mod parser_test {
    use crate::{lexer::Lexer, Parser};
    use x_util::LevelFilter::Debug;

    fn init() {
        x_util::env_logger::builder()
            .is_test(true)
            .filter_level(Debug)
            .init();
    }

    #[test]
    fn function_test() {
        init();
        let raw_str = r#"function a(a b)"#;
        parser(raw_str);
    }

    #[test]
    fn command_test() {
        init();
        let raw_str = r#"a"#;
        parser(raw_str);
        let raw_str = r#""a""#;
        parser(raw_str);
    }

    fn parser(s: &str) {
        let lexer = Lexer::new(s.chars());
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().unwrap();
        println!(
            "{}",
            parser
                .output
                .iter()
                .map(|s| { s.to_string() })
                .collect::<Vec<String>>()
                .join("")
        );
        println!("{:?}", ast);
    }
}
