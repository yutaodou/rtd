use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::db::storage;
use crate::task::Priority;
use crate::view::single;
use std::io::stdout;
use std::str::FromStr;

#[derive(Debug)]
pub struct Edit {
    task_id: u32,
    title: Option<String>,
    list: Option<String>,
    priority: Option<Priority>,
}

impl Edit {
    pub fn new(args: &ArgMatches) -> Edit {
        let task_id: u32;
        let mut title = None;
        let mut list = None;
        let mut priority = None;
        let mut free_args = vec![];

        args.values_of("INPUT").unwrap().for_each(|arg| {
            if arg.starts_with(':') && arg.len() > 1 {
                list = Some(arg.get(1..arg.len()).unwrap().to_string());
            } else if arg.starts_with('+') && arg.len() > 1 {
                priority = Some(
                    arg.get(1..arg.len())
                        .map(|p| Priority::from_str(p).unwrap())
                        .unwrap(),
                );
            } else {
                free_args.push(arg);
            }
        });

        if free_args.is_empty() {
            panic!("Todo id not provided.");
        }

        task_id = free_args.get(0).unwrap().parse().unwrap();

        if free_args.len() == 2 {
            title = Some((*free_args.get(1).unwrap()).to_string());
        }

        Edit {
            task_id,
            title,
            list,
            priority,
        }
    }
}

impl Command for Edit {
    fn run(self) -> Result<(), String> {
        let result = storage::get(self.task_id)
            .and_then(move |mut task| {
                if let Some(new_list) = self.list {
                    task.list = new_list;
                }

                if let Some(new_priority) = self.priority {
                    task.priority = new_priority;
                }

                if let Some(new_title) = self.title {
                    task.title = new_title;
                }
                Ok(task)
            })
            .and_then(|update_task| {
                storage::update(&update_task).and_then(|task| single::render(task, &mut stdout()))
            });

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Failed to edit task")),
        }
    }
}
