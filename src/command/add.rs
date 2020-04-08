use std::result::Result;

use crate::command::Command;
use crate::storage;
use crate::task::{Priority, Task};

#[derive(Debug)]
pub struct Add {
    title: String,
    list: String,
    priority: Priority,
}

impl Add {
    fn parse(args: &[String]) -> Add {
        let mut title = String::new();
        let mut list = String::from("inbox");
        let mut priority = Priority::Medium;
        for arg in args {
            if arg.starts_with("~") && arg.len() > 1 {
                list = arg.get(1..arg.len()).unwrap().to_string();
            } else if arg.starts_with("!") && arg.len() > 1 {
                priority = arg
                    .get(1..arg.len())
                    .map(|p| Priority::from(p).unwrap())
                    .unwrap();
            } else {
                title = arg.clone();
            }
        }

        Add {
            title,
            list,
            priority,
        }
    }

    pub fn new(args: &[String]) -> Result<Add, &'static str> {
        if args.len() < 2 {
            Err("expect at least 2 arguments: 'rtd add <to do>'")
        } else {
            Ok(Add::parse(args))
        }
    }
}

impl Command for Add {
    fn run(self: &Self) -> Result<(), &'static str> {
        let new_task = Task::new(self.title.clone(), self.list.clone());
        let result = storage::add(&new_task)?;
        println!("{:?}", result);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::command::Add;
    use crate::task::Priority;

    #[test]
    fn test_parse() {
        let args = vec![
            "add".to_string(),
            "todo-title".to_string(),
            "~list".to_string(),
            "!L".to_string(),
        ];

        let add = Add::new(args.as_slice()).unwrap();

        assert_eq!(add.title, "todo-title");
        assert_eq!(add.list, "list");
        assert_eq!(add.priority, Priority::Low);
    }
}
