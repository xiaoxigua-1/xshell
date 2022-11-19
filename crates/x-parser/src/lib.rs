mod lexer;
mod tokens;

use lexer::Lexer;
use x_input::Input;
use x_protocol::{shell_err::Result, InputState, Output};

pub fn parser(input: &mut Input) -> Result<Output> {
    let input_str = input.user_input.clone();
    match input.state {
        InputState::Execute | InputState::NewLine => input.clear(),
        _ => {}
    }

    Ok(Output::new(format!("{} ", input_str)))
}
