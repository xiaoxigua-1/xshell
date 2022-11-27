mod syntax;
mod lexer;

use std::{iter::Peekable, fmt::Display, ops::Range};

use lexer::Lexer;
use x_protocol::{Token, Tokens, Kwd, crossterm::style::StyledContent};
use x_input::Input;
use x_protocol::{
    shell_err::Result,
    InputState, Output, ast::AST, crossterm::style::Stylize, ShellErr,
};

pub fn repl(input: &mut Input) -> Result<Output> {
    let user_input = input.user_input.clone();
    let mut lexer = Lexer::new(user_input.chars());
    let raw_input = input.user_input.clone();
    let parser = Parser::new(lexer, raw_input);
    
    match input.state {
        InputState::NewLine => input.clear(),
        _ => {}
    }

    Ok(Output::new(format!(" ")))
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    raw_input: String,
    output: String,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, raw_input: String) -> Self {
        Parser { lexer: lexer.peekable(), raw_input, output: String::new() }
    }

    fn parse(&mut self) -> Result<Option<AST>> {
        Ok(if let Some(token) = self.lexer.next() {
            let token = token?;
            self.output_str(&token.ty.default_highlighter());
            Some(match &token.ty {
                Tokens::Keyword(k) => self.builtin(&k)?,
                _ => {
                    if let Some(peek_token) = self.lexer.peek() {
                        if let Ok(peek_token) = peek_token {
                            if peek_token.ty.eq(&Tokens::Symbol('(')) {
                                return Ok(Some(self.call_function(token)?));
                            }
                        }
                    }
                    
                    self.command(token)?
                }
            })
        } else {
            None
        })
    }

    fn builtin(&mut self, kwd: &Kwd) -> Result<AST> {
        Ok(match kwd {
            Kwd::Function => self.function_syntax()?
        })
    }

    fn output_str<T: Display>(&mut self, s: T) {
        self.output.push_str(&s.to_string());
    }

    fn eat_token_eq<F, E, S>(&mut self, eq_func: F, err: E, custom_style: Option<S>) -> Result<Token>
        where F: FnOnce(&Token) -> bool,
        E: FnOnce(Range<usize>) -> ShellErr,
        S: FnOnce(String) -> StyledContent<String>
    {
        self.eat_whitespace()?;
        let token = self.lexer.next();        
        if let Some(token) = token {
            let token = token?;
            if eq_func(&token) {
                let output = if let Some(highlighter) = custom_style {
                    token.ty.highlighter(highlighter)
                } else {
                    token.ty.default_highlighter()
                };
                self.output_str(output);
                Ok(token)
            } else {
                Err(err(token.span))
            }
        } else {
            Err(ShellErr::EOF)
        }
    }

    fn eat_token_eq_default<F>(&mut self, eq_func: F, err_message: &str) -> Result<Token>
        where F: FnOnce(&Token) -> bool
    {
        self.eat_token_eq(eq_func, |span| { ShellErr::Syntax(span, err_message.into()) }, None::<Box<dyn FnOnce(String) -> StyledContent<String>>>)
    }

    fn eat_token_eq_custom_err<F, E>(&mut self, eq_func: F, err: E) -> Result<Token>
        where F: FnOnce(&Token) -> bool, E: FnOnce(Range<usize>) -> ShellErr
    {
        self.eat_token_eq(eq_func, err, None::<Box<dyn FnOnce(String) -> StyledContent<String>>>)
    }

    fn eat_token_eq_custom_color<F, S>(&mut self, eq_func: F, err_message: &str, custom_style: S) -> Result<Token>
        where F: FnOnce(&Token) -> bool, S: FnOnce(String) -> StyledContent<String> 
    {
        self.eat_token_eq(eq_func, |span| { ShellErr::Syntax(span, err_message.into()) }, Some(custom_style))
    }


    fn eat_whitespace(&mut self) -> Result<()> {
        loop {
            if let Some(t) = self.lexer.peek() {
                match t {
                    Ok(t) => match t.ty {
                        Tokens::Space(c) => {
                            self.output.push(c);
                            self.lexer.next()
                        }
                        _ => break Ok(())
                    },
                    Err(_) => break Ok(())
                };
            } else {
                break Ok(());
            };
        }
    }
}

#[cfg(test)]
mod parser_test {
    use crate::{lexer::Lexer, Parser};
    use x_util::LevelFilter::Debug;

    fn init() {
        x_util::env_logger::builder().is_test(true).filter_level(Debug).init();
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
        let mut parser = Parser::new(lexer, s.to_string());
        let ast = parser.parse().unwrap();
        println!("{}", parser.output);
        println!("{:?}", ast);
    }
}
