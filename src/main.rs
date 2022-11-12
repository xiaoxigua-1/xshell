use x_engine::events::XShellEvent;

fn main() {
    let mut xshell_event = XShellEvent::default();
    xshell_event.listen_start();
}
