use x_engine::{Command, BuiltinCommand};

macro_rules! create_command {
    ($commands: ident, $name: literal, $usage: literal, $func: expr) => {
        let command = BuiltinCommand::new($name.into(), $usage.into(), Box::new($func));
        $commands.push(Box::new(command));
    };
}

pub fn get_commands() -> Vec<Box<dyn Command>> {
    let mut commands: Vec<Box<dyn Command>> = vec![];
    create_command!(
        commands,
        "exit",
        "",
        |_| {
            Ok(())
        }
    );
    commands
}

