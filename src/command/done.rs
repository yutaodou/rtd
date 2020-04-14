use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::storage;
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
    fn run(self: &Self) -> Result<(), &'static str> {
        self.args.values_of("INPUT").unwrap().for_each(|id| {
            let mut task_id = id;
            let mut done = true;
            if task_id.starts_with("~") {
                task_id = &task_id[1..];
                done = false;
            };
            let result = storage::done(task_id.parse().unwrap(), done).unwrap();
            single::render(&result, &mut stdout()).unwrap();
        });
        Ok(())
    }
}
