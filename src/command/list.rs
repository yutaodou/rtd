use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::storage;
use crate::task::Task;
use crate::view::list;

#[derive(Debug)]
pub struct List<'a> {
    opts: ArgMatches<'a>,
}

impl<'a> List<'a> {
    pub fn new(opts: ArgMatches) -> Result<List, &'static str> {
        Ok(List { opts })
    }
}

impl<'a> Command for List<'a> {
    fn run(self: &Self) -> Result<(), &'static str> {
        let tasks = storage::get_all().unwrap();
        let mut result: Vec<&Task>;

        if self.opts.is_present("done") {
            result = tasks.iter().filter(|task| task.done).collect();
        } else if !self.opts.is_present("all") {
            result = tasks.iter().filter(|task| task.done == false).collect();
        } else {
            result = tasks.iter().collect();
        }

        result = match self.opts.value_of("name") {
            Some(name) => result
                .iter()
                .filter(|task| task.list == name)
                .map(|task| *task)
                .collect(),
            None => result,
        };

        let render = list::Render { tasks: result };
        render.render(&mut stdout())?;

        Ok(())
    }
}
