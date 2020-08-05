use std::result::Result;

use clap::ArgMatches;

use crate::command::{Command, MultiProcessor};
use crate::db::storage;
use crate::model::Task;
use crate::view::single;

pub struct Done<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> Done<'a> {
    pub fn new(args: &'a ArgMatches<'a>) -> Done<'a> {
        Done { args }
    }
}

impl MultiProcessor<Task> for Done<'_> {
    fn process_single(&self, task_id: u32) -> Result<Task, String> {
        let mark_as_done = !self.args.is_present("unset");
        storage::get(task_id)
            .and_then(|mut task| {
                if mark_as_done {
                    task.mark_completed();
                } else {
                    task.mark_uncompleted();
                }
                Ok(task)
            })
            .and_then(|task| storage::update(&task))
    }
}

impl<'a> Command for Done<'a> {
    fn run(self: Self) -> Result<(), String> {
        self.process(
            self.args.values_of("INPUT").unwrap().collect(),
            Box::new(|task| single::render(&task).unwrap()),
        )
    }
}
