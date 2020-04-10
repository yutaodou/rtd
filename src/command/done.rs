use crate::command::Command;
use crate::storage;
use crate::view::single;
use std::io::stdout;
use std::result::Result;

pub struct Done<'a> {
    args: &'a [String],
}

impl<'a> Done<'a> {
    pub fn new(args: &'a [String]) -> Result<Done, &'static str> {
        if args.len() < 2 {
            Err("expect at least 2 arguments: 'rtd done <todo-id>'")
        } else {
            Ok(Done { args })
        }
    }
}

impl<'a> Command for Done<'a> {
    fn run(self: &Self) -> Result<(), &'static str> {
        let result = storage::done(self.args[1].parse().unwrap())?;
        single::render(&result, &mut stdout())?;
        Ok(())
    }
}
