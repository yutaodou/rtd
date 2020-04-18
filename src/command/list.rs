use std::io::stdout;
use std::result::Result;

use clap::ArgMatches;

use crate::command::Command;
use crate::storage;
use crate::task::{Task, SMART_LISTS};
use crate::view::list;

#[derive(Debug)]
pub struct List<'a> {
    opts: &'a ArgMatches<'a>,
}

impl<'a> List<'a> {
    pub fn new(opts: &'a ArgMatches) -> List<'a> {
        List { opts }
    }
}

impl<'a> Command for List<'a> {
    fn run(self: Self) -> Result<(), &'static str> {
        let tasks = storage::get_all().unwrap();
        let result: Vec<&Task>;

        if self.opts.is_present("done") {
            result = tasks.iter().filter(|task| task.done).collect();
        } else if !self.opts.is_present("all") {
            result = tasks.iter().filter(|task| task.done == false).collect();
        } else {
            result = tasks.iter().collect();
        }

        match self.opts.value_of("name") {
            Some(name) => render_list(
                &result,
                name,
                SMART_LISTS.contains(&name.to_lowercase().as_str()),
            ),
            None => render_lists(&result),
        }
    }
}

fn render_lists(tasks: &Vec<&Task>) -> Result<(), &'static str> {
    render_list(tasks, "today", true).unwrap();

    let mut lists: Vec<&str> = tasks.iter().map(|task| task.list.as_str()).collect();
    lists.sort();
    lists.dedup();

    let mut result = lists.iter().map(|list| render_list(tasks, list, false));

    if result.any(|result| result.is_err()) {
        Err("Failed to show tasks")
    } else {
        Ok(())
    }
}

fn render_list(result: &Vec<&Task>, list: &str, is_smart_list: bool) -> Result<(), &'static str> {
    let tasks = result
        .iter()
        .filter(|task| task.is_in_list(list))
        .map(|task| *task)
        .collect::<Vec<&Task>>();

    let render = list::Render {
        tasks: &tasks,
        list,
        is_smart_list,
    };
    render.render(&mut stdout())
}
