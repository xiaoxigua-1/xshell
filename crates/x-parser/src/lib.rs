mod syntax;
mod lexer;

use std::{iter::Peekable, fmt::Display};

use lexer::Lexer;
use x_protocol::{Token, Tokens, Kwd};
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
            Some(match &token.ty {
                Tokens::Keyword(k) => self.builtin(&k)?,
                _ => self.command(token)?
            })
        } else {
            None
        })
    }

    fn builtin(&mut self, kwd: &Kwd) -> Result<AST> {
        self.output_str(kwd.to_string().blue());
        Ok(match kwd {
            Kwd::Function => self.function_syntax()?
        })
    }

    fn command(&mut self, command: Token) -> Result<AST> {
        Ok(AST::Command {  })
    }

    fn output_str<T: Display>(&mut self, s: T) {
        self.output.push_str(&s.to_string());
    }

    fn eat_token_eq<F>(&mut self, eq_func: F, err_message: &str) -> Result<Token>
    where F: FnOnce(&Token) -> bool
    {
        self.eat_whitespace()?;
        let token = self.lexer.next();        
        if let Some(token) = token {
            let token = token?;
            if eq_func(&token) {
                Ok(token)
            } else {
                Err(ShellErr::Syntax(token.span, err_message.to_string()))
            }
        } else {
            Err(ShellErr::EOF)
        }
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


    #[test]
    fn function_test() {
        let raw_string = r#"function a(a b)"#;
        parser(raw_string);
    }

    fn parser(s: &str) {
        let lexer = Lexer::new(s.chars());
        let mut parser = Parser::new(lexer, s.to_string());
        let ast = parser.parse().unwrap();

        println!("{:?}", ast);
    }
}
