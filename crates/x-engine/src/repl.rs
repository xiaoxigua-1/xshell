use x_input::Input;
use x_parser::{Lexer, Parser};
use x_protocol::Output;

pub fn repl(input: &mut Input) -> String {
    let raw_input = input.user_input.clone();
    let lexer = Lexer::new(raw_input.chars());
    let mut parser = Parser::new(lexer);

    loop {
        match parser.parse() {
            Ok(token) => {
                if let Some(token) = token {
                    
                } else {
                    break;
                }
            }
            Err(e) => {

            }
        }
    }

    format!("{} ", parser.output)
}
