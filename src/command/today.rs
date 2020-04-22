use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::db::storage;
use crate::task::Task;
use crate::view::single;

pub struct Today<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> Today<'a> {
    pub fn new(args: &'a ArgMatches<'a>) -> Today<'a> {
        Today { args }
    }
}

impl<'a> Command for Today<'a> {
    fn run(self: Self) -> Result<(), String> {
        let mark_for_today = !self.args.is_present("unset");
        let results = self
            .args
            .values_of("INPUT")
            .unwrap()
            .map(|task_id| process(task_id, mark_for_today));

        let mut captured_error = None;
        results.for_each(|result| match result {
            Ok(task) => single::render(&task, &mut stdout()).unwrap(),
            Err(err) => captured_error = Some(err),
        });

        match captured_error {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }
}

fn process(input: &str, mark_for_today: bool) -> Result<Task, String> {
    match input.parse() {
        Err(_) => Err(format!("Invalid task id: {}", input)),
        Ok(task_id) => storage::get(task_id)
            .and_then(|mut task| {
                if mark_for_today {
                    task.mark_for_today();
                } else {
                    task.remove_from_today();
                }
                Ok(task)
            })
            .and_then(|task| match storage::update(&task) {
                Ok(_) => Ok(task),
                Err(error) => Err(error),
            }),
    }
}
