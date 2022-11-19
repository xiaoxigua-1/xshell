use std::{iter::{Peekable, Enumerate}, str::Chars, ops::Range};

use x_protocol::Result;

use crate::tokens::{Tokens, Token};

pub struct Lexer<'a> {
    input_stream: Peekable<Enumerate<Chars<'a>>>,
    end: Range<usize>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        let end = chars.clone().count();
        Lexer { input_stream: chars.enumerate().peekable(), end: end..end }
    }
    
    /// Get next token from input stream
    pub fn next_token(&mut self) -> Result<Token> {
        Ok(if let Some((i, c)) = self.input_stream.next() {
            match c {
                c if c.is_whitespace() => Token::new(Tokens::Space(c), i..i),
                '"' | '\'' => self.str_lex(i, c == '"')?,
                '0'..='9' => self.int_lex((i, c))?,
                _ => self.ident_lex((i, c))?,

            } 
        } else {
            Token::new(Tokens::EOF, self.end.clone())
        })
    }

    fn str_lex(&mut self, start: usize, double: bool) -> Result<Token> {
        let mut s = String::new();

        loop {
            if let Some((i, c)) = self.input_stream.next() {
                match c {
                    c if double && c == '"' => break Ok(Token::new(Tokens::Str(s), start..i)),
                    c if !double && c == '\'' => break Ok(Token::new(Tokens::Str(s), start..i)),
                    _ => s.push(c)
                }
            } else {
                break Err(x_protocol::ShellErr::Syntax(start..start, "unterminated string".into()));
            }
        }
    }

    fn int_lex(&mut self, (i, c): (usize, char)) -> Result<Token> {
        let mut int_s = String::from(c);

        match c {
            '0' => if let Some((_, c)) = self.input_stream.peek() {
                    match c {
                        'b' => self.binary_lex(&mut int_s)?,
                        _ => self.decimal_lex(&mut int_s)?,
                    }
                }
            _ => self.decimal_lex(&mut int_s)?,
        }

        Ok(Token::new(Tokens::Int(int_s.clone()), i..(i + int_s.len() - 1)))
    }

    fn decimal_lex(&mut self, s: &mut String) -> Result<()> {
        loop {
            if let Some((_, c)) = self.input_stream.peek() {
                match c {
                    '0'..='9' => {
                        let (_, c) = self.input_stream.next().unwrap();
                        s.push(c);
                    }
                    _ => break Ok(())
                }
            } else {
                break Ok(());
            }
        }
    }

    fn binary_lex(&mut self, s: &mut String) -> Result<()> {
        self.input_stream.next();
        s.push('b');
        loop {
            if let Some((_, c)) = self.input_stream.peek() {
                match c {
                    '0'..='1' => {
                        let (_, c) = self.input_stream.next().unwrap();
                        s.push(c);
                    },
                    _ => break Ok(())
                }
            } else {
                break Ok(());
            }
        }
    }

    fn ident_lex(&mut self, (start, c): (usize, char)) -> Result<Token> {
        let mut s = String::from(c);

        loop {
            if let Some((i, c)) = self.input_stream.peek() {
                match c {
                    c if !c.is_ascii_punctuation() && !c.is_whitespace() || c == &'_' => {
                        let (_, c) = self.input_stream.next().unwrap();
                        s.push(c);
                    },
                    _ => break Ok(Token::new(Tokens::Ident(s), start..(i - 1)))
                }
            } else {
                break Ok(Token::new(Tokens::Ident(s), start..(self.end.end - 1)));
            }
        }
    }
}

#[cfg(test)]
mod test_lexer {
    use Tokens::*;

    use crate::tokens::Tokens;

    use super::Lexer;

    #[test]
    fn test_number() {
        let s = r#"0b1101 123"#;
        let mut lexer = Lexer::new(s.chars());
        let assert_token_arr = [Int("0b1101".into()), Space(' '), Int("123".into()), EOF];
        
        for assert_token in assert_token_arr {
            assert_eq!(assert_token, lexer.next_token().unwrap().ty);
        }
    }
}
