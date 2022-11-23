mod lexer;
mod tokens;

use std::iter::Peekable;

use lexer::Lexer;
use tokens::{Token, Tokens, Kwd};
use x_input::Input;
use x_protocol::{
    shell_err::Result,
    InputState, Output, ast::AST,
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
                Tokens::Keyword(k) => self.builtin(k)?,
                _ => self.command(token)?
            })
        } else {
            None
        })
    }

    fn builtin(&mut self, kwd: &Kwd) -> Result<AST> {
        self.eat_whitespace()?;
        
        Ok(AST::Function)
    }

    fn command(&mut self, command: Token) -> Result<AST> {
        Ok(AST::Function)
    }

    fn eat_whitespace(&mut self) -> Result<()> {
        loop {
            if let Some(t) = self.lexer.peek() {
                match t {
                    Ok(t) => match t.ty {
                        Tokens::Space(_) => self.lexer.next(),
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
