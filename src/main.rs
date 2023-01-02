mod cli;
mod builtin_commands;

use std::env::vars;

use builtin_commands::get_commands;
use clap::Parser;
use x_engine::{ShellState, XShellEvent};

fn main() {
    let args: cli::Args = cli::Args::parse();
    let mut xshell_state = ShellState::default();

    // init commands
    let commands = get_commands();
    xshell_state.init_commands(commands);

    // set environment variable
    args.envs
        .iter()
        .for_each(|(key, value)| xshell_state.add_env(key.clone(), value.clone()));
    for (key, value) in vars() {
        println!("{} {}", key, value);
        xshell_state.add_env(key, value);
    }

    let mut xshell_event = XShellEvent::new(xshell_state);
    xshell_event.listen_start().unwrap();
}
