use crate::model::Task;
use crate::prettytable::{format, row, Table};
use ansi_term::Colour::Red;
use ansi_term::{ANSIGenericString, Style};
use std::io::{ Write};

use crate::model::SMART_LISTS;

pub struct Render<'a> {
    pub tasks: &'a Vec<&'a Task>,
    pub list: Option<&'a str>
}

impl<'a> Render<'a> {
    pub fn render<W: Write>(self: &Self, w: &mut W) -> Result<(), String> {
        let max_title_width = *self
            .tasks
            .iter()
            .map(|task| task.title.len())
            .max()
            .get_or_insert(0);

        match self.list {
            Some(list) => self.render_list(w, self.tasks, list, max_title_width),
            None => self.render_lists(w, max_title_width),
        }

        Ok(())
    }

    fn render_lists<W: Write>(self: &Self, w: &mut W, max_title_width: usize) {
        let mut lists: Vec<&str> = self.tasks.iter().map(|task| task.list.as_str()).collect();
        lists.sort();
        lists.dedup();

        for list in lists.iter() {
            let tasks = self
                .tasks
                .iter()
                .filter(|task| task.is_in_list(list))
                .cloned()
                .collect::<Vec<&Task>>();

            self.render_list(w, &tasks, list, max_title_width);
        }
    }

    fn render_list<W: Write>(self: &Self, w: &mut W, tasks: &[&Task], list: &str, max_title_width: usize) {
        if tasks.is_empty() {
            writeln!(w, "No tasks found for {}", list).unwrap();
        } else {
            let is_smart_list = SMART_LISTS.contains(&list.to_lowercase().as_str());

            writeln!(w, "{}", list).unwrap();
            let mut table = Table::new();
            for task in tasks.iter() {
                table.add_row(row![
                    task_id(task),
                    title(task, max_title_width),
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

fn task_id(task: &Task) -> String {
    format!("{}.", task.id)
}

fn priority(task: &Task) -> String {
    format!("+{}", task.priority.to_string())
}

fn title(task: &Task, max_title_width: usize) -> ANSIGenericString<str> {
    if task.done {
        Style::new().strikethrough()
    } else {
        Style::default()
    }
    .paint(format!("{:width$}", &task.title, width= max_title_width))
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
