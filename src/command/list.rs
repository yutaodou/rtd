use crate::command::Command;
use crate::storage;
use std::result::Result;

#[derive(Debug)]
pub struct List {}

impl List {
    pub fn new() -> Result<List, &'static str> {
        Ok(List {})
    }
}

impl Command for List {
    fn exec(self: &Self) -> Result<(), &'static str> {
        let tasks = storage::get_all().unwrap();
        for task in tasks.iter() {
            if !task.done {
                println!(
                    "Id: {}, title: {}, done: {}",
                    task.id, task.title, task.done
                )
            }
        }

        Ok(())
    }
}
