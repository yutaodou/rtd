use clap::ArgMatches;
use std::io::stdout;
use std::result::Result;

use crate::command::Command;
use crate::storage;
use crate::task::{Priority, Task};
use crate::view::single;

#[derive(Debug)]
pub struct Add {
    title: String,
    list: String,
    priority: Priority,
}

impl Add {
    fn parse(args: &ArgMatches) -> Add {
        let mut title = String::new();
        let mut list = String::from("inbox");
        let mut priority = Priority::Medium;
        args.values_of("INPUT").unwrap().for_each(|arg| {
            if arg.starts_with("~") && arg.len() > 1 {
                list = arg.get(1..arg.len()).unwrap().to_string();
            } else if arg.starts_with("!") && arg.len() > 1 {
                priority = arg
                    .get(1..arg.len())
                    .map(|p| Priority::from(p).unwrap())
                    .unwrap();
            } else {
                title = arg.to_string();
            }
        });

        Add {
            title,
            list,
            priority,
        }
    }

    pub fn new(args: &ArgMatches) -> Result<Add, &'static str> {
        Ok(Add::parse(args))
    }
}

impl Command for Add {
    fn run(self: &Self) -> Result<(), &'static str> {
        let new_task = Task::new(self.title.clone(), self.list.clone());
        let result = storage::add(&new_task)?;
        single::render(&result, &mut stdout())?;
        Ok(())
    }
}
