use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::db::storage;
use crate::model::Task;
use crate::view::list;

#[derive(Debug)]
pub struct List<'a> {
    opts: &'a ArgMatches<'a>,
}

impl<'a> List<'a> {
    pub fn new(opts: &'a ArgMatches) -> List<'a> {
        List { opts }
    }
}

impl<'a> Command for List<'a> {
    fn run(self: Self) -> Result<(), String> {
        let tasks = storage::get_all().unwrap();
        let result: Vec<&Task>;

        if self.opts.is_present("done") {
            result = tasks.iter().filter(|task| task.done).collect();
        } else if !self.opts.is_present("all") {
            result = tasks.iter().filter(|task| !task.done).collect();
        } else {
            result = tasks.iter().collect();
        }

        let render = list::Render {
            tasks: &result,
            list: self.opts.value_of("name"),
        };
        render.render(&mut stdout())
    }
}
