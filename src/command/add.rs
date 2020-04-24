use clap::ArgMatches;
use std::io::stdout;
use std::result::Result;

use crate::command::todo_args::ToDoArgs;
use crate::command::Command;
use crate::db::storage;
use crate::task::{Priority, Task};
use crate::view::single;

#[derive(Debug)]
pub struct Add {
    args: ToDoArgs,
}

impl Add {
    pub fn new(args: &ArgMatches) -> Add {
        Add {
            args: ToDoArgs::parse(args),
        }
    }
}

impl Command for Add {
    fn run(self: Self) -> Result<(), String> {
        let args = self.args;
        let title = args.free_args.get(0);
        if title == None {
            return Err("Missing title for todo.".to_string());
        }

        let new_task = Task::new(
            title.map(|a| a.to_string()).unwrap(),
            args.list.clone().unwrap_or_else(|| "inbox".to_string()),
            args.parse_priority()?.or(Some(Priority::Medium)).unwrap(),
            args.parse_due_date()?,
        );
        let result = storage::add(&new_task)?;
        single::render(&result, &mut stdout())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::command::todo_args::ToDoArgs;

    #[test]
    fn test_parse_due_date() {
        let mut add = ToDoArgs {
            list: None,
            due_date: Some("20200202".to_string()),
            priority: None,
            free_args: vec![],
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
