use crate::command::Command;
use crate::storage;
use crate::view::single;
use clap::ArgMatches;
use std::io::stdout;
use std::result::Result;

pub struct Done<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> Done<'a> {
    pub fn new(args: &'a ArgMatches<'a>) -> Result<Done, &'static str> {
        Ok(Done { args })
    }
}

impl<'a> Command for Done<'a> {
    fn run(self: &Self) -> Result<(), &'static str> {
        self.args.values_of("INPUT").unwrap().for_each(|id| {
            let result = storage::done(id.parse().unwrap()).unwrap();
            single::render(&result, &mut stdout()).unwrap();
        });

        Ok(())
    }
}
