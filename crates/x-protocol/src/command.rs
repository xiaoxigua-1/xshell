use crate::example::Example;

pub trait Command {
    fn get_name(&self) -> &'static str;

    fn get_example(&self) -> Vec<Example> {
        vec![]
    }

    fn get_usage(&self) -> &'static str;

    fn run(&self);

    fn is_sub(&self) -> bool;
}
