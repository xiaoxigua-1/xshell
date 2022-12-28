mod cli;

use x_engine::XShellEvent;

fn main() {
    let mut xshell_event = XShellEvent::default();
    xshell_event.listen_start().unwrap();
}
