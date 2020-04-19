use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::storage;
use crate::task::Task;
use crate::view::single;

pub struct Today<'a> {
    args: &'a ArgMatches<'a>,
}

impl<'a> Today<'a> {
    pub fn new(args: &'a ArgMatches<'a>) -> Today<'a> {
        Today { args }
    }
}

impl<'a> Command for Today<'a> {
    fn run(self: Self) -> Result<(), String> {
        let results = self.args.values_of("INPUT").unwrap().map(process);

        let mut has_errors = false;
        results.for_each(|result| match result {
            Ok(task) => single::render(&task, &mut stdout()).unwrap(),
            Err(err) => {
                has_errors = true;
                println!("{}", err);
            }
        });

        if false {
            Err(String::from("Failed to add task to today"))
        } else {
            Ok(())
        }
    }
}

fn process(input: &str) -> Result<Task, String> {
    parse(input)
        .and_then(|(task_id, marked_for_today)| {
            storage::get(task_id).and_then(|mut task| {
                if marked_for_today {
                    task.mark_for_today();
                } else {
                    task.remove_from_today();
                }
                Ok(task)
            })
        })
        .and_then(|task| match storage::update(&task) {
            Ok(_) => Ok(task),
            Err(error) => Err(error),
        })
}

fn parse(value: &str) -> Result<(u32, bool), String> {
    let mut task_id = value;
    let mut marked_for_today = true;
    if value.starts_with('~') {
        task_id = &value[1..];
        marked_for_today = false;
    };
    match task_id.parse() {
        Ok(id) => Ok((id, marked_for_today)),
        Err(_) => Err(format!("Unknown task id: '{}'", task_id)),
    }
}

#[cfg(test)]
mod test {
    use crate::command::today::parse;

    #[test]
    fn test_parse() {
        let (id, marked_for_today) = parse("~123").unwrap();
        assert_eq!(id, 123);
        assert_eq!(marked_for_today, false);

        let (id, marked_for_today) = parse("123").unwrap();
        assert_eq!(id, 123);
        assert_eq!(marked_for_today, true);

        let result = parse("asdf");
        assert_eq!(result.is_err(), true);
        assert_eq!(result.err(), Some(String::from("Unknown task id: 'asdf'")));
    }
}
