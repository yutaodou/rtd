use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::command::MultiProcessor;
use crate::db::storage;
use crate::model::Task;
use crate::view::single;

pub struct Today<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> Today<'a> {
    pub fn new(args: &'a ArgMatches<'a>) -> Today<'a> {
        Today { args }
    }
}

impl MultiProcessor<Task> for Today<'_> {
    fn process_single(&self, task_id: u32) -> Result<Task, String> {
        let mark_for_today = !self.args.is_present("unset");
        storage::get(task_id)
            .and_then(|mut task| {
                if mark_for_today {
                    task.mark_for_today();
                } else {
                    task.remove_from_today();
                }
                Ok(task)
            })
            .and_then(|task| storage::update(&task))
    }
}

impl<'a> Command for Today<'a> {
    fn run(self: Self) -> Result<(), String> {
        self.process(
            self.args.values_of("INPUT").unwrap().collect(),
            Box::new(|task| single::render(&task, &mut stdout()).unwrap()),
        )
    }
}
