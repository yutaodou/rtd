use clap::ArgMatches;
use std::str::FromStr;

use crate::model::{Priority, SmartDate};

#[derive(Debug)]
pub struct ToDoArgs {
    pub list: Option<String>,
    pub mark_for_today: bool,
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub free_args: Vec<String>,
}

impl ToDoArgs {
    pub fn parse_priority(&self) -> Result<Option<Priority>, String> {
        match &self.priority {
            None => Ok(None),
            Some(input) => Priority::from_str(input.as_str()).map(Some),
        }
    }

    pub fn parse_due_date(self: &Self) -> Result<Option<SmartDate>, String> {
        match &self.due_date {
            None => Ok(None),
            Some(input) => SmartDate::from_str(input).map(Some),
        }
    }

    pub fn parse(args: &ArgMatches) -> ToDoArgs {
        let extract = |arg: &str| arg.get(1..arg.len()).map(|a| a.to_string());

        let mut list = None;
        let mut priority = None;
        let mut due_date = None;
        let mut free_args = vec![];
        let mark_for_today = args.is_present("today");

        args.values_of("INPUT").unwrap().for_each(|arg| {
            if arg.starts_with(':') && arg.len() > 1 {
                list = extract(arg);
            } else if arg.starts_with('+') && arg.len() > 1 {
                priority = extract(arg);
            } else if arg.starts_with('@') && arg.len() > 1 {
                due_date = extract(arg);
            } else {
                free_args.push(arg.to_string());
            }
        });

        ToDoArgs {
            list,
            mark_for_today,
            priority,
            due_date,
            free_args,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::todo_args::ToDoArgs;
    use crate::model::SmartDate;
    use std::str::FromStr;

    #[test]
    fn test_parse_due_date() {
        let mut add = ToDoArgs {
            list: None,
            mark_for_today: false,
            due_date: Some("20200202".to_string()),
            priority: None,
            free_args: vec![],
        };
        assert_eq!(
            add.parse_due_date().unwrap(),
            Some(SmartDate::from_str("20200202").unwrap())
        );

        add.due_date = None;
        assert_eq!(add.parse_due_date().unwrap(), None);
    }
}
