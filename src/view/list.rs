use ansi_term::Colour::Red;
use ansi_term::{ANSIGenericString, Style};
use std::io::{Error, Write};
use time::OffsetDateTime;

use crate::task::Task;

pub struct Render<'a> {
    pub tasks: &'a Vec<&'a Task>,
    pub list: &'a str,
    pub is_smart_list: bool,
}

impl<'a> Render<'a> {
    pub fn render<W: Write>(self: &Self, w: &mut W) -> Result<(), String> {
        if self.tasks.is_empty() {
            match writeln!(w, "No tasks found for {}", self.list) {
                Err(_) => Err(String::from("Failed to show list tasks")),
                Ok(_) => Ok(()),
            }
        } else {
            writeln!(w, "{}", self.list).unwrap();
            let mut results = self.tasks.iter().map(|task| self.render_single(w, task));
            match results.find(|result| result.is_err()) {
                Some(Err(err)) => Err(err.to_string()),
                _ => Ok(()),
            }
        }
    }

    fn render_single<W: Write>(self: &Self, w: &mut W, task: &Task) -> Result<(), Error> {
        writeln!(
            w,
            "{:>4}. {} +{} {} {}",
            task.id,
            title(task),
            default_style(task.priority.to_string().as_str()),
            list(task, self.is_smart_list),
            due_date(task)
        )
    }
}

fn default_style(content: &str) -> ANSIGenericString<str> {
    Style::default().paint(content)
}

fn title(task: &Task) -> ANSIGenericString<str> {
    if task.done {
        Style::new().strikethrough()
    } else {
        Style::default()
    }
    .paint(&task.title)
}

fn list(task: &Task, is_smart_list: bool) -> ANSIGenericString<str> {
    Style::default().paint(if is_smart_list {
        format!(":{}", task.list)
    } else {
        String::from("")
    })
}

fn due_date(task: &Task) -> ANSIGenericString<str> {
    task.due_date
        .map_or(ANSIGenericString::from(""), |due_date| {
            if due_date.lt(&OffsetDateTime::now().date()) && !task.done {
                Style::default().fg(Red)
            } else {
                Style::default()
            }
            .paint(format!("@{}", due_date.format("%F")))
        })
}
