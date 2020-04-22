use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::db::storage;
use crate::task::Task;
use crate::view::single;

pub struct Done<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> Done<'a> {
    pub fn new(args: &'a ArgMatches<'a>) -> Done<'a> {
        Done { args }
    }
}

impl<'a> Command for Done<'a> {
    fn run(self: Self) -> Result<(), String> {
        let mark_as_done = !self.args.is_present("unset");
        let results = self
            .args
            .values_of("INPUT")
            .unwrap()
            .map(|task_id| process(task_id, mark_as_done));

        let mut captured_error = None;
        results.for_each(|result| match result {
            Ok(task) => single::render(&task, &mut stdout()).unwrap(),
            Err(err) => {
                captured_error = Some(err);
            }
        });

        match captured_error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}

fn process(input: &str, mark_as_done: bool) -> Result<Task, String> {
    match input.parse() {
        Err(_) => Err(format!("Invalid task id: {}", input)),
        Ok(task_id) => storage::get(task_id)
            .and_then(|mut task| {
                if mark_as_done {
                    task.mark_completed();
                } else {
                    task.mark_uncompleted();
                }
                Ok(task)
            })
            .and_then(|task| match storage::update(&task) {
                Ok(_) => Ok(task),
                Err(error) => Err(error),
            }),
    }
}
