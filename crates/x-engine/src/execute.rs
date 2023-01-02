use std::sync::Arc;

use x_protocol::{ast::{AST, Expression}, ShellState};

pub fn execute(state: &mut ShellState, asts: Vec<AST>) {
    for ast in asts {
        match ast {
            AST::Command { name, args } => command(state, name.ty.to_string(), args),
            _ => {}
        }
    } 
}

fn command(state: &mut ShellState, name: String, args: Vec<Expression>) {
    let Some(command) = state.commands.iter().find(|command| { command.get_name() == name }) else {
        return;
    };

    command.clone().run(state, args.iter().map(|e| { e.to_string() }).collect());
}
