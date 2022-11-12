use x_input::Input;
use x_protocol::{Output, shell_err, InputState};

pub fn parser(input: &mut Input) -> shell_err::Result<Output> {
    let output = input.user_input.clone();
    match input.state {
        InputState::Execute | InputState::NewLine => input.clear(),
        _ => {}
    }

    Ok(Output::new(output))
} 
