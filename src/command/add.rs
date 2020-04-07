use crate::command::Command;
use crate::storage;
use crate::task::Task;
use std::result::Result;

#[derive(Debug)]
pub struct Add<'a> {
    args: &'a [String],
}

impl<'a> Add<'a> {
    pub fn new(args: &'a [String]) -> Result<Add, &'static str> {
        if args.len() <= 2 {
            Err("expect at least 3 arguments: 'rtd add <to do>'")
        } else {
            Ok(Add { args: args })
        }
    }
}

impl<'a> Command for Add<'a> {
    fn exec(self: &Self) -> Result<(), &'static str> {
        let new_task = Task::new(self.args[2].clone());
        let result = storage::add(&new_task)?;
        println!(
            "Id: {}, title: {}, done: {}",
            result.id, result.title, result.done
        );
        Ok(())
    }
}
