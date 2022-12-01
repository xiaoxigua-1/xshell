use x_input::Input;
use x_parser::{Lexer, Parser};
use x_protocol::{
    crossterm::style::{StyledContent, Stylize},
    crossterm::Result,
    ShellErr, ShellState,
};
use x_render::Render;

pub fn repl(render: &mut Render, input: &mut Input, shell_state: &ShellState) -> Result<()> {
    use x_protocol::InputState::*;
    let raw_input = input.user_input.clone();
    let lexer = Lexer::new(raw_input.chars());
    let mut parser = Parser::new(lexer);
    let mut output: Vec<StyledContent<String>> = vec![];


    let is_error = loop {
        match parser.parse() {
            Ok(token) => {
                if let Some(token) = token {
                } else {
                    break false;
                }

                output.append(&mut parser.output.clone());
            }
            Err(e) => {
                output.append(&mut parser.output.clone());
                let Some(out) = error_header(e.clone(), &raw_input, &mut output, &mut parser) else {
                    break true;
                };
                output.push(out);
                output.push(format!("{:?}", e).stylize());
                break true;
            }
        }
    };
    let output_str = format!(
        "{} ",
        output
            .iter()
            .map(|s| { s.to_string() })
            .collect::<Vec<_>>()
            .join("")
    );
    let cursor_index = {
        let mut index = 0;

        input.user_input[input.cursor..input.user_input.len()]
            .chars()
            .for_each(|c| {
                index += if c.is_ascii() { 1 } else { 2 };
            });
        index + 1
    };

    render.clear_line()?;
    render.render(output_str, cursor_index)?;
    match input.state {
        Execute => {
            render.new_line(shell_state)?;
            if is_error {
                input.state = NONE;
                repl(render, input, shell_state)?;
            } else {
                input.clear();
            }
        },
        NewLine => {
            input.clear();
            render.new_line(shell_state)?;
        }
        _ => {}
    }
    Ok(())
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
