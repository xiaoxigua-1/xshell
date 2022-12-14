use x_checker::Checker;
use x_input::Input;
use x_parser::{Lexer, Parser};
use x_protocol::{
    ast::AST,
    crossterm::style::{StyledContent, Stylize},
    crossterm::Result,
    ShellErr, ShellState,
};
use x_render::Render;
use crate::execute::execute;

// read eval print loop
pub fn repl(render: &mut Render, input: &mut Input, shell_state: &mut ShellState) -> Result<()> {
    use x_protocol::InputState::*;
    let raw_input = input.user_input.clone();
    let lexer = Lexer::new(raw_input.chars());
    let checker = Checker::new(shell_state);
    let mut parser = Parser::new(lexer);
    let mut output: Vec<StyledContent<String>> = vec![];
    let mut asts: Vec<AST> = vec![];

    let is_error = loop {
        match parser.parse() {
            Ok(ast) => {
                output.append(&mut parser.output.clone());

                if let Some(ast) = ast {
                    if let Err(e) = checker.check(&ast) {
                        error_header(e, &raw_input, &mut output, &mut parser);
                    };
                    asts.push(ast);
                } else {
                    break false;
                }
            }
            Err(e) => {
                output.append(&mut parser.output.clone());
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
            if is_error {
                input.state = NONE;
                repl(render, input, shell_state)?;
            } else {
                input.clear();
                // check ast and run ast
                execute(shell_state, render, asts);
            }
            if !shell_state.is_exit {
                render.new_line(shell_state)?;
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
        ShellErr::Syntax(range, _) => {
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
        ShellErr::UnterminatedStr(range) => output.push(
            format!(
                "{}{}",
                raw_input[range.clone()].red(),
                raw_input[range.end..].green()
            )
            .stylize(),
        ),
        ShellErr::Unterminated(_, i, _) | ShellErr::UnknownCommand(i, _) => output[i] = output[i].clone().red(),
        _ => {}
    }
}
