use x_input::Input;
use x_parser::{Lexer, Parser};
use x_protocol::{
    ast::AST,
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
    let mut asts: Vec<AST> = vec![];

    let is_error = loop {
        match parser.parse() {
            Ok(ast) => {
                if let Some(ast) = ast {
                    asts.push(ast);
                } else {
                    break false;
                }

                output = parser.output.clone();
            }
            Err(e) => {
                output = parser.output.clone();
                render.debug(format!("{:?}", e))?;
                error_header(e.clone(), &raw_input, &mut output, &mut parser);
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
            render.debug(format!("{:?}", asts))?;
            render.new_line(shell_state)?;
            if is_error {
                input.state = NONE;
                repl(render, input, shell_state)?;
            } else {
                input.clear();
                // check ast and run ast
            }
        }
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
) {
    match e {
        x_protocol::ShellErr::Syntax(range, _) => {
            output.push(raw_input[range].to_string().red());

            loop {
                match parser.eat_remaining_token() {
                    Ok(t) => output.push(t),
                    Err(e) => {
                        if let ShellErr::EOF = e {
                            break;
                        } else {
                            error_header(e, raw_input, output, parser)
                        }
                    }
                }
            }
        }
        x_protocol::ShellErr::UnterminatedStr(range) => output.push(
            format!(
                "{}{}",
                raw_input[range.clone()].red(),
                raw_input[range.end..].green()
            )
            .stylize(),
        ),
        x_protocol::ShellErr::Unterminated(_, i, _) => output[i] = output[i].clone().red(),
        x_protocol::ShellErr::EOF => {}
    }
}
