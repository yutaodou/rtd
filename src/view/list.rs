use crate::model::Task;
use crate::prettytable::{format, Table};
use std::io::Write;

use crate::model::SMART_LISTS;
use super::formatter::Formatter;

pub struct Render<'a> {
    pub tasks: &'a Vec<&'a Task>,
    pub list: Option<&'a str>,
}

impl<'a> Render<'a> {
    pub fn render<W: Write>(self: &Self, w: &mut W) -> Result<(), String> {
        match self.list {
            Some(list) => self.render_list(w, list, None),
            None => self.render_lists(w),
        }

        Ok(())
    }

    fn render_lists<W: Write>(self: &Self, w: &mut W) {
        let mut lists: Vec<&str> = self.tasks.iter().map(|task| task.list.as_str()).collect();
        lists.sort();
        lists.dedup();

        for list in lists.iter() {
            self.render_list(w, list, Some(max_title_width(self.tasks)));
        }
    }

    fn render_list<W: Write>(self: &Self, w: &mut W, list: &str, max_width: Option<usize>) {
        let tasks = self
            .tasks
            .iter()
            .filter(|task| task.is_in_list(list))
            .cloned()
            .collect::<Vec<&Task>>();

        if tasks.is_empty() {
            writeln!(w, "No tasks found for {}", list).unwrap();
        } else {
            let is_smart_list = SMART_LISTS.contains(&list.to_lowercase().as_str());
            let width: usize = max_width.unwrap_or(max_title_width(&tasks));

            writeln!(w, "{}", list).unwrap();
            let mut table = Table::new();
            for task in tasks.iter() {
                let formatter = Formatter::new(task);
                table.add_row(row![
                    formatter.task_id(),
                    formatter.done(),
                    formatter.title(),
                    formatter.priority(),
                    formatter.due_date(),
                    formatter.list(is_smart_list)
                ]);
            }

            let format = format::FormatBuilder::new().padding(1, 1).build();
            table.set_format(format);
            table.printstd();
        }
    }
}

fn max_title_width(tasks: &Vec<&Task>) -> usize {
    *tasks
        .iter()
        .map(|task| task.title.len())
        .max()
        .get_or_insert(0)
}