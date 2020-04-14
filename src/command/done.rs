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
            let (task_id, done) = parse(id).unwrap();
            let result = storage::done(task_id, done).unwrap();
            single::render(&result, &mut stdout()).unwrap();
        });
        Ok(())
    }
}

fn parse(value: &str) -> Result<(u32, bool), &'static str> {
    let mut task_id = value;
    let mut done = true;
    if value.starts_with("~") {
        task_id = &value[1..];
        done = false;
    };
    let id = task_id.parse().unwrap();
    Ok((id, done))
}
