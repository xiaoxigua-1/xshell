use std::ops::Range;

pub type Result<T> = std::result::Result<T, ShellErr>;

#[derive(Debug, Clone)]
pub enum ShellErr {
    Syntax(Range<usize>, String),
    Unterminated(Range<usize>, usize, String),
    UnterminatedStr(Range<usize>),
    EOF,
}
