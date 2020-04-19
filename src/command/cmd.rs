use std::result::Result;

pub trait Command {
    fn run(self: Self) -> Result<(), String>;
}
