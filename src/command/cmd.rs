use std::result::Result;

pub trait Command {
    fn exec(self: &Self) -> Result<(), &'static str>;
}