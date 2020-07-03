use crate::model::Task;
use crate::prettytable::{format, row, Table};
use ansi_term::Colour::Red;
use ansi_term::{ANSIGenericString, Style};
use std::io::Write;

use crate::model::SMART_LISTS;

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
                table.add_row(row![
                    task_id(task),
                    done(task),
                    title(task, width),
                    priority(task),
                    due_date(task),
                    task_list(task, is_smart_list)
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

fn done(task: &Task) -> String {
    if task.done {
        String::from("✔")
    } else {
        String::from("")
    }
}

fn task_id(task: &Task) -> String {
    format!("{}.", task.id)
}

fn priority(task: &Task) -> String {
    format!("+{}", task.priority.to_string())
}

fn title(task: &Task, max_title_width: usize) -> String {
    format!("{:width$}", &task.title, width = max_title_width)
}

fn task_list(task: &Task, is_smart_list: bool) -> ANSIGenericString<str> {
    Style::default().paint(if is_smart_list {
        format!(":{}", task.list)
    } else {
        String::from("")
    })
}

fn due_date(task: &Task) -> ANSIGenericString<str> {
    let due_date = task
        .due_date
        .as_ref()
        .map_or_else(|| "".to_string(), |due_date| format!("@{}", due_date));

    if task.is_overdue() {
        Style::default().fg(Red)
    } else {
        Style::default()
    }
    .paint(due_date)
}
