use std::result::Result;

use clap::ArgMatches;

use crate::command::{Command, MultiProcessor};
use crate::db::storage;

pub struct Delete<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> Delete<'a> {
    pub fn new(args: &'a ArgMatches<'a>) -> Delete<'a> {
        Delete { args }
    }
}

impl MultiProcessor<u32> for Delete<'_> {
    fn process_single(&self, task_id: u32) -> Result<u32, String> {
        storage::delete(task_id)
    }
}

impl<'a> Command for Delete<'a> {
    fn run(self: Self) -> Result<(), String> {
        self.process(
            self.args.values_of("INPUT").unwrap().collect(),
            Box::new(|task_id| println!("{}", format!("Task '{}' deleted", task_id))),
        )
    }
}
