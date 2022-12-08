use std::{fmt::Display, ops::Range};

use crossterm::style::{StyledContent, Stylize};

macro_rules! Gen {
    ($name: ident, $($kwd: ident => $str: expr),*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum $name {
            $(
                $kwd
            ),*
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$kwd => write!(f, "{}", $str),
                    )*
                }
            }
        }

        impl $name {
            pub fn new(s: &str) -> Option<Self> {
                match s {
                    $(
                        $str => Some(Self::$kwd),
                    )*
                    _ => None
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Ident(String),
    Keyword(Kwd),
    Symbol(char),
    Str(String),
    Path(String),
    Int(String),
    Space(char),
    Arg(String),
    And,
    Or,
    PipeLine,
    Background,
    NewLine,
    EOF,
}

Gen!(
    Kwd,
    Function => "def"
);

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: Tokens,
    pub span: Range<usize>,
}

impl Token {
    pub fn new(ty: Tokens, span: Range<usize>) -> Self {
        Token { ty, span }
    }

    pub fn eq(&self, ty: Tokens) -> bool {
        self.ty == ty
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tokens::*;

        write!(
            f,
            "{}",
            match self {
                Path(s) | Ident(s) | Int(s) | Str(s) | Arg(s) => s.to_string(),
                Keyword(k) => k.to_string(),
                Space(c) | Symbol(c) => c.to_string(),
                Background => "&".into(),
                And => "&&".into(),
                Or => "|".into(),
                PipeLine => "||".into(),
                NewLine => "\n".into(),

                _ => "".into(),
            }
        )
    }
}

impl Tokens {
    pub fn default_highlighter(&self) -> StyledContent<String> {
        use Tokens::*;

        match self {
            Ident(s) | Int(s) => s.clone().dark_blue(),
            Str(s) => s.clone().dark_green(),
            Keyword(k) => k.to_string().dark_green().bold(),
            Space(c) => c.to_string().reset(),
            Symbol(c) => c.to_string().with(crossterm::style::Color::Rgb {
                r: 242,
                g: 133,
                b: 0,
            }),
            Arg(s) => s.clone().yellow(),
            _ => self.to_string().reset(),
        }
    }

    pub fn highlighter<F>(&self, highlighter: F) -> StyledContent<String>
    where
        F: FnOnce(String) -> StyledContent<String>,
    {
        highlighter(self.to_string())
    }
}
