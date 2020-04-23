use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::db::storage;
use crate::task::{Priority, Task};
use crate::view::single;
use std::str::FromStr;
use time::Date;

#[derive(Debug)]
pub struct Add {
    title: String,
    list: String,
    priority: Priority,
    due_date: Option<Date>,
}

impl Add {
    pub fn new(args: &ArgMatches) -> Add {
        let mut title = String::new();
        let mut list = String::from("inbox");
        let mut priority = Priority::Medium;
        let mut due_date = None;
        args.values_of("INPUT").unwrap().for_each(|arg| {
            if arg.starts_with(':') && arg.len() > 1 {
                list = arg.get(1..arg.len()).unwrap().to_string();
            } else if arg.starts_with('+') && arg.len() > 1 {
                priority = arg
                    .get(1..arg.len())
                    .map(|p| Priority::from_str(p).unwrap())
                    .unwrap();
            } else if arg.starts_with('@') && arg.len() > 1 {
                due_date = arg.get(1..arg.len()).and_then(parse_due_date);
            } else {
                title = arg.to_string();
            }
        });

        Add {
            title,
            list,
            priority,
            due_date,
        }
    }
}

impl Command for Add {
    fn run(self: Self) -> Result<(), String> {
        let new_task = Task::new(
            self.title.clone(),
            self.list.clone(),
            self.priority,
            self.due_date
        );
        let result = storage::add(&new_task)?;
        single::render(&result, &mut stdout())?;
        Ok(())
    }
}

fn parse_due_date(due_date: &str) -> Option<Date> {
    Date::parse(due_date, "%F")
        .or(Date::parse(due_date, "%-Y%m%d"))
        .ok()
}

#[cfg(test)]
mod test {
    use crate::command::add::parse_due_date;

    #[test]
    fn test_parse_due_date() {
        assert_eq!(parse_due_date("20200202").unwrap(), time::date!(2020-02-02));
        assert_eq!(parse_due_date("2020-02-02").unwrap(), time::date!(2020-02-02));
        assert_eq!(parse_due_date("2020"), None);
    }
}