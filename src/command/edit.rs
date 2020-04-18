use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::storage;
use crate::task::Priority;
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
        let mut unknown = vec![];
        args.values_of("INPUT").unwrap().for_each(|arg| {
            if arg.starts_with("~") && arg.len() > 1 {
                list = Some(arg.get(1..arg.len()).unwrap().to_string());
            } else if arg.starts_with("!") && arg.len() > 1 {
                priority = Some(
                    arg.get(1..arg.len())
                        .map(|p| Priority::from_str(p).unwrap())
                        .unwrap(),
                );
            } else {
                unknown.push(arg);
            }
        });

        if unknown.len() < 1 {
            panic!("Todo id not provided.");
        }

        task_id = unknown.get(0).unwrap().parse().unwrap();

        if unknown.len() == 2 {
            title = Some(unknown.get(1).unwrap().to_string());
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
    fn run(self) -> Result<(), &'static str> {
        let result = storage::get(self.task_id)
            .and_then(move |mut task| {
                self.list.map(|new_list| task.list = new_list);
                self.priority
                    .map(|new_priority| task.priority = new_priority);
                self.title.map(|new_title| task.title = new_title);
                Ok(task)
            })
            .and_then(|update_task| storage::update(&update_task));

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to edit task"),
        }
    }
}
