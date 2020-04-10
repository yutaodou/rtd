use crate::command::Command;
use crate::storage;
use crate::view::list;
use std::io::stdout;
use std::result::Result;

#[derive(Debug)]
pub struct List {}

impl List {
    pub fn new() -> Result<List, &'static str> {
        Ok(List {})
    }
}

impl Command for List {
    fn run(self: &Self) -> Result<(), &'static str> {
        let tasks = storage::get_all().unwrap();
        let render = list::Render { tasks };
        render.render(&mut stdout())?;

        Ok(())
    }
}
