use std::{
    iter::{Enumerate, Peekable},
    ops::Range,
    str::Chars,
};

use x_protocol::{Result, ShellErr};

use x_protocol::{Kwd, Token, Tokens};

pub struct Lexer<'a> {
    input_stream: Peekable<Enumerate<Chars<'a>>>,
    end: Range<usize>,
    is_eof: bool,
}

fn token_type(s: String) -> Tokens {
    if let Some(kwd) = Kwd::new(&s) {
        Tokens::Keyword(kwd)
    } else {
        Tokens::Ident(s)
    }
}

impl<'a> Lexer<'a> {
    /// # Create a new Lexer.
    /// ## Example
    /// ```
    /// let mut s = r#"123abc"#;
    /// let lexer = Lexer::new(s.chars());
    /// ```
    pub fn new(chars: Chars<'a>) -> Self {
        let end = chars.clone().count();
        Lexer {
            input_stream: chars.enumerate().peekable(),
            end: end..end,
            is_eof: false,
        }
    }

    /// Get next token from input stream
    pub fn next_token(&mut self) -> Result<Token> {
        Ok(if let Some((i, c)) = self.input_stream.next() {
            match c {
                '\n' => Token::new(Tokens::NewLine, i..i),
                c if c.is_whitespace() => Token::new(Tokens::Space(c), i..i),
                '"' | '\'' => self.str_lex((i, c), c == '"')?,
                '|' => self.or(i),
                '&' => self.and(i),
                // path
                '.' | '/' | '~' => self.path((i, c))?,
                c if c.is_ascii_punctuation() && c != '_' => Token::new(Tokens::Symbol(c), i..i),
                '0'..='9' => self.int_lex((i, c))?,
                _ => self.ident_lex((i, c))?,
            }
        } else {
            self.is_eof = true;
            Token::new(Tokens::EOF, self.end.clone())
        })
    }

    fn or(&mut self, start: usize) -> Token {
        if let Some((end, _)) = self.input_stream.next_if(|(_, c)| c.eq(&'|')) {
            Token::new(Tokens::PipeLine, start..end)
        } else {
            Token::new(Tokens::Or, start..start)
        }
    }

    fn and(&mut self, start: usize) -> Token {
        if let Some((end, _)) = self.input_stream.next_if(|(_, c)| c.eq(&'&')) {
            Token::new(Tokens::And, start..end)
        } else {
            Token::new(Tokens::Background, start..start)
        }
    }

    fn path(&mut self, (start, c): (usize, char)) -> Result<Token> {
        let mut path = String::from(c);
        let mut end = start.clone();

        loop {
            if let Some((i, c)) = self.input_stream.next_if(|(_, c)| {
                !c.is_whitespace()
                    && !c.eq(&'|')
                    && !c.eq(&'<')
                    && !c.eq(&'>')
                    && !c.eq(&':')
                    && !c.eq(&'"')
                    && !c.eq(&'?')
                    && !c.eq(&'*')
            }) {
                end += if c.eq(&'\\') {
                    path.push(self.escape_char()?);
                    i + 1
                } else {
                    i
                };
                path.push(c);
            } else {
                break;
            }
        }

        Ok(Token::new(Tokens::Path(path), start..end))
    }

    fn escape_char(&mut self) -> Result<char> {
        if let Some((_, c)) = self.input_stream.next() {
            Ok(match c {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '0' => '\0',
                c @ _ => c,
            })
        } else {
            Err(ShellErr::EOF)
        }
    }

    fn str_lex(&mut self, (start, c): (usize, char), double: bool) -> Result<Token> {
        let mut s = String::from(c);

        loop {
            if let Some((i, c)) = self.input_stream.next() {
                s.push(c);
                if (double && c == '"') || (!double && c == '\'') {
                    break Ok(Token::new(Tokens::Str(s), start..i));
                }
            } else {
                break Err(x_protocol::ShellErr::Unterminated(
                    start..start,
                    "Unterminated string".into(),
                ));
            }
        }
    }

    fn int_lex(&mut self, (i, c): (usize, char)) -> Result<Token> {
        let mut int_s = String::from(c);

        if c == '0' {
            if let Some((_, c)) = self.input_stream.peek() {
                match c {
                    // Binary number.
                    'b' => self.binary_lex(i, &mut int_s)?,
                    // Hex number.
                    'x' => self.hex_lex(i, &mut int_s)?,
                    // Decimal number.
                    _ => self.decimal_lex(i, &mut int_s)?,
                }
            }
        } else {
            self.decimal_lex(i, &mut int_s)?
        }

        Ok(Token::new(
            Tokens::Int(int_s.clone()),
            i..(i + int_s.len() - 1),
        ))
    }

    fn decimal_lex(&mut self, start: usize, s: &mut String) -> Result<()> {
        loop {
            if let Some((_, c)) = self.input_stream.peek() {
                match c {
                    '0'..='9' | '.' => {
                        let (_, c) = self.input_stream.next().unwrap();
                        s.push(c);
                    }
                    c if c.is_ascii_punctuation() || c.is_whitespace() => break Ok(()),
                    _ => {
                        let end = self.eat(start.clone(), |c| {
                            !c.is_whitespace() && !c.is_ascii_punctuation()
                        });
                        break Err(ShellErr::Syntax(start..end, "".into()));
                    }
                }
            } else {
                break Ok(());
            }
        }
    }

    fn hex_lex(&mut self, start: usize, s: &mut String) -> Result<()> {
        self.input_stream.next();
        s.push('x');
        loop {
            if let Some((_, c)) = self.input_stream.peek() {
                match c {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        let (_, c) = self.input_stream.next().unwrap();
                        s.push(c);
                    }
                    c if c.is_ascii_punctuation() || c.is_whitespace() => break Ok(()),
                    _ => {
                        let end = self.eat(start.clone(), |c| {
                            !c.is_whitespace() && !c.is_ascii_punctuation()
                        });
                        break Err(ShellErr::Syntax(start..end, "".into()));
                    }
                }
            } else {
                break Ok(());
            }
        }
    }

    fn binary_lex(&mut self, start: usize, s: &mut String) -> Result<()> {
        self.input_stream.next();
        s.push('b');
        loop {
            if let Some((_, c)) = self.input_stream.peek() {
                match c {
                    '0'..='1' => {
                        let (_, c) = self.input_stream.next().unwrap();
                        s.push(c);
                    }
                    c if c.is_ascii_punctuation() || c.is_whitespace() => break Ok(()),
                    _ => {
                        let end = self.eat(start.clone(), |c| {
                            !c.is_whitespace() && !c.is_ascii_punctuation()
                        });
                        break Err(ShellErr::Syntax(start..end, "".into()));
                    }
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
                if !c.is_ascii_punctuation() && !c.is_whitespace() || c.eq(&'_') || c.eq(&'-') {
                    let (_, c) = self.input_stream.next().unwrap();
                    s.push(c);
                } else {
                    break Ok(Token::new(token_type(s), start..(i - 1)));
                }
            } else {
                break Ok(Token::new(token_type(s), start..(self.end.end - 1)));
            }
        }
    }

    /// Eat
    fn eat<F>(&mut self, start: usize, func: F) -> usize
    where
        F: FnOnce(&char) -> bool + Copy,
    {
        let mut end = start;

        loop {
            if let Some((i, _)) = self.input_stream.next_if(|(_, c)| func(c)) {
                end = i;
            } else {
                break;
            }
        }

        end + 1
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_eof {
            Some(self.next_token())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_lexer {
    use x_protocol::{Tokens, Tokens::*};

    use super::Lexer;

    #[test]
    fn test_number() {
        let s = r#"0b1101 123"#;
        let assert_token_arr = [Int("0b1101".into()), Space(' '), Int("123".into()), EOF];

        assert_token(s, &assert_token_arr);
    }

    #[test]
    fn test_ident() {
        let s = r#"abc_1 cc123 你好 __A"#;
        let assert_token_arr = [
            Ident("abc_1".into()),
            Space(' '),
            Ident("cc123".into()),
            Space(' '),
            Ident("你好".into()),
            Space(' '),
            Ident("__A".into()),
            EOF,
        ];
        assert_token(s, &assert_token_arr);
    }

    #[test]
    fn test_string() {
        let s = r#""abc"'abc'"#;
        let assert_token_arr = [Str(r#""abc""#.into()), Str(r#"'abc'"#.into()), EOF];

        assert_token(s, &assert_token_arr);
    }

    #[test]
    fn test_symbol() {
        let s = r#"()"#;
        let assert_token_arr = [Symbol('('), Symbol(')'), EOF];

        assert_token(s, &assert_token_arr);
    }

    #[test]
    fn test_call() {
        let s = r#"a(c)"#;
        let assert_token_arr = [
            Ident("a".into()),
            Symbol('('),
            Ident("c".into()),
            Symbol(')'),
            EOF,
        ];

        assert_token(s, &assert_token_arr);
    }

    #[test]
    fn test_and_or() {
        let s = r#"|||&&&"#;
        let assert_token_arr = [PipeLine, Or, And, Background, EOF];

        assert_token(s, &assert_token_arr);
    }

    #[test]
    fn test_path() {
        let s = r#"./a%b-c#@!_a/b|||"#;
        let assert_token_arr = [Path("./a%b-c#@!_a/b".into()), PipeLine, Or];

        assert_token(s, &assert_token_arr);
    }

    fn assert_token(s: &str, arr: &[Tokens]) {
        let mut lexer = Lexer::new(s.chars());

        for assert_token in arr {
            assert_eq!(assert_token.clone(), lexer.next_token().unwrap().ty);
        }
    }
}
