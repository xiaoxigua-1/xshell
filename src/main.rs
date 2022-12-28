mod cli;

use clap::Parser;
use x_engine::XShellEvent;

fn main() {
    let args = cli::Args::parse();
    let mut xshell_event = XShellEvent::default();
    xshell_event.listen_start().unwrap();
}
