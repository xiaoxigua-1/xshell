use x_protocol::{ast::{AST, Expression}, ShellState, crossterm::terminal::{disable_raw_mode, enable_raw_mode}};
use x_render::Render;

pub fn execute(state: &mut ShellState, render: &mut Render, asts: Vec<AST>) {
    for ast in asts {
        match ast {
            AST::Command { name, args } => command(state, render, name.ty.to_string(), args),
            _ => {}
        }
    } 
}

fn command(state: &mut ShellState, render: &mut Render, name: String, args: Vec<Expression>) {
    let Some(command) = state.commands.iter().find(|command| { command.get_name() == name }) else {
        return;
    };
    
    disable_raw_mode().unwrap();
    command.clone().run(state, args.iter().map(|e| { e.to_string() }).collect());
    enable_raw_mode().unwrap();
}
