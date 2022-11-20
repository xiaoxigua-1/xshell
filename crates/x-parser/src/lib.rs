mod lexer;
mod tokens;

use lexer::Lexer;
use tokens::{Token, Tokens};
use x_input::Input;
use x_protocol::{
    crossterm::style::Stylize,
    shell_err::Result,
    InputState, Output, ShellErr,
};

pub fn parser(input: &mut Input) -> Result<Output> {
    let user_input = input.user_input.clone();
    let mut lexer = Lexer::new(user_input.chars());
    let mut output = String::new();

    match input.state {
        InputState::Execute | InputState::NewLine => input.clear(),
        _ => {}
    }

    loop {
        match lexer.next_token() {
            Ok(token) => {
                output.push_str(&highlighter(&token));
                if token.ty == Tokens::EOF {
                    break;
                }
            }
            Err(e) => match e {
                ShellErr::Unterminated(range, _) => output.push_str(&format!(
                    "{}{}",
                    &user_input[range.start..range.start + 1].red(),
                    &user_input[range.start + 1..].green()
                )),
                ShellErr::Syntax(range, _) => {
                    output.push_str(&format!("{}", &user_input[range].red()))
                }
            },
        }
    }

    Ok(Output::new(format!("{} ", output)))
}

fn highlighter(token: &Token) -> String {
    match token.ty.clone() {
        Tokens::Str(s) => s.green(),
        Tokens::Int(i) => i.blue(),
        Tokens::Space(s) => s.to_string().reset(),
        Tokens::Ident(ident) => ident.reset(),
        Tokens::EOF => "".to_string().reset(),
    }
    .to_string()
}
