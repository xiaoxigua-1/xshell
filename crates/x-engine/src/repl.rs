use x_input::Input;
use x_parser::{Lexer, Parser};
use x_protocol::{
    crossterm::style::{StyledContent, Stylize},
    ShellErr,
};

pub fn repl(input: &mut Input) -> String {
    let raw_input = input.user_input.clone();
    let lexer = Lexer::new(raw_input.chars());
    let mut parser = Parser::new(lexer);
    let mut output: Vec<StyledContent<String>> = vec![];

    loop {
        match parser.parse() {
            Ok(token) => {
                if let Some(token) = token {
                } else {
                    break;
                }

                output.append(&mut parser.output.clone());
            }
            Err(e) => {
                output.append(&mut parser.output.clone());
                let Some(out) = error_header(e.clone(), &raw_input, &mut output, &mut parser) else {
                    break;
                };
                output.push(out);
                output.push(format!("{:?}", e).stylize())
            }
        }
    }

    format!(
        "{} ",
        output
            .iter()
            .map(|s| { s.to_string() })
            .collect::<Vec<_>>()
            .join("")
    )
}

fn error_header(
    e: ShellErr,
    raw_input: &String,
    output: &mut Vec<StyledContent<String>>,
    parser: &mut Parser,
) -> Option<StyledContent<String>> {
    match e {
        x_protocol::ShellErr::Syntax(range, _) => Some(raw_input[range].to_string().red()),
        x_protocol::ShellErr::UnterminatedStr(range) => Some(
            format!(
                "{}{}",
                raw_input[range.clone()].red(),
                raw_input[range.end..].green()
            )
            .stylize(),
        ),
        x_protocol::ShellErr::Unterminated(_, i, _) => {
            output[i] = output[i].clone().red();
            if let Err(e) = parser.eat_remaining_tokens() {
                error_header(e, raw_input, output, parser);
            }
            None
        }
        x_protocol::ShellErr::EOF => None,
    }
}
