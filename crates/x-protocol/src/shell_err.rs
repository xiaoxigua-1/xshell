use std::ops::Range;

pub type Result<T> = std::result::Result<T, ShellErr>;

#[derive(Debug, Clone)]
pub enum ShellErr {
    Syntax(Range<usize>, String)
}
