use std::result::Result;

use crate::command::Command;
use crate::storage;
use crate::task::Task;

#[derive(Debug)]
pub struct Add {
    title: String,
    list: String,
}

impl Add {
    fn parse(args: &[String]) -> Add {
        let mut title = String::new();
        let mut list = String::new();
        for arg in args {
            if arg.starts_with("~") && arg.len() > 1 {
                list = arg.get(1..arg.len()).unwrap().to_string();
            } else {
                title = arg.clone();
            }
        }

        if list.is_empty() {
            list = "inbox".to_string();
        }

        Add { title, list }
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
    fn exec(self: &Self) -> Result<(), &'static str> {
        let new_task = Task::new(self.title.clone(), self.list.clone());
        let result = storage::add(&new_task)?;
        println!("{:?}", result);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Add;

    #[test]
    fn test_new() {
        let args = vec![
            "add".to_string(),
            "todo-title".to_string(),
            "~list".to_string(),
        ];

        let add = Add::new(args.as_slice()).unwrap();

        assert_eq!(add.title, "todo-title");
        assert_eq!(add.list, "list");
    }
}
