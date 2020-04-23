use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::db::storage;
use crate::task::{Priority, Task};
use crate::view::single;
use std::borrow::Borrow;
use std::str::FromStr;
use time::Date;

#[derive(Debug)]
pub struct Add {
    title: Option<String>,
    list: Option<String>,
    priority: Option<String>,
    due_date: Option<String>,
}

impl Add {
    pub fn new(args: &ArgMatches) -> Add {
        let extract = |arg: &str| arg.get(1..arg.len()).map(|a| a.to_string());

        let mut title = None;
        let mut list = None;
        let mut priority = None;
        let mut due_date = None;
        args.values_of("INPUT").unwrap().for_each(|arg| {
            if arg.starts_with(':') && arg.len() > 1 {
                list = extract(arg);
            } else if arg.starts_with('+') && arg.len() > 1 {
                priority = extract(arg);
            } else if arg.starts_with('@') && arg.len() > 1 {
                due_date = extract(arg);
            } else {
                title = Some(arg.to_string());
            }
        });

        Add {
            title,
            list,
            priority,
            due_date,
        }
    }

    fn parse_priority(self: &Self) -> Result<Priority, String> {
        match &self.priority {
            None => Ok(Priority::default()),
            Some(input) => Priority::from_str(input.as_str()),
        }
    }

    fn parse_due_date(self: &Self) -> Result<Option<Date>, String> {
        match &self.due_date {
            None => Ok(None),
            Some(input) => Date::parse(&input, "%F")
                .or(Date::parse(&input, "%-Y%m%d"))
                .map(|date| Some(date))
                .map_err(|_| format!("Invalid due date: {}", input)),
        }
    }
}

impl Command for Add {
    fn run(self: Self) -> Result<(), String> {
        let add = self.borrow();
        if add.title == None {
            return Err("Missing title for todo.".to_string());
        }

        let new_task = Task::new(
            self.title.clone().unwrap(),
            self.list.clone().unwrap_or("inbox".to_string()),
            self.parse_priority()?,
            self.parse_due_date()?,
        );
        let result = storage::add(&new_task)?;
        single::render(&result, &mut stdout())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::command::Add;

    #[test]
    fn test_parse_due_date() {
        let mut add = Add {
            title: None,
            list: None,
            due_date: Some("20200202".to_string()),
            priority: None,
        };
        assert_eq!(
            add.parse_due_date().unwrap(),
            Some(time::date!(2020 - 02 - 02))
        );

        add.due_date = Some("2020-02-02".to_string());
        assert_eq!(
            add.parse_due_date().unwrap(),
            Some(time::date!(2020 - 02 - 02))
        );

        add.due_date = Some("2020".to_string());
        assert_eq!(
            add.parse_due_date().unwrap_err(),
            "Invalid due date: 2020".to_string()
        );

        add.due_date = None;
        assert_eq!(add.parse_due_date().unwrap(), None);
    }
}
