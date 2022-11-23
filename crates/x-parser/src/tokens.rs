use std::{ops::Range, fmt::Display};

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
    Symbol(Symbol),
    Str(String),
    Int(String),
    Space(char),
    NewLine,
    EOF,
}

Gen!(
    Kwd,
    Function => "function"
);

Gen!(
    Symbol,
    Dot => "." 
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
}
