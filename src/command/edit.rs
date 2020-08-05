use clap::ArgMatches;
use std::borrow::Borrow;
use std::result::Result;

use crate::command::{Command, ToDoArgs};
use crate::db::storage;
use crate::view::single;

#[derive(Debug)]
pub struct Edit {
    args: ToDoArgs,
}

impl Edit {
    pub fn new(args: &ArgMatches) -> Edit {
        Edit {
            args: ToDoArgs::parse(args),
        }
    }
}

impl Command for Edit {
    fn run(self: Self) -> Result<(), String> {
        let args = self.args;

        let task_id = match args.free_args.get(0) {
            None => Err("Task id required.".to_string()),
            Some(input) => input.parse::<u32>().map_err(|a| a.to_string()),
        }?;

        storage::get(task_id)
            .and_then(|mut task| {
                if let Some(new_list) = args.list.borrow() {
                    task.list = new_list.to_string();
                }

                if let Some(new_title) = args.free_args.get(1) {
                    task.title = new_title.to_string();
                }

                args.parse_priority().and_then(|priority| match priority {
                    Some(new_priority) => {
                        task.priority = new_priority;
                        Ok(())
                    }
                    _ => Ok(()),
                })?;

                args.parse_due_date().and_then(|due_date| match due_date {
                    Some(new_due_date) => {
                        task.due_date = Some(new_due_date);
                        Ok(())
                    }
                    _ => Ok(()),
                })?;

                Ok(task)
            })
            .and_then(|update_task| {
                storage::update(&update_task).and_then(|task| single::render(&task))
            })
    }
}
