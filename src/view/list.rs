use std::io::{Error, Write};

use ansi_term::Style;

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
                _ => Ok(())
            }
        }
    }

    fn render_single<W: Write>(self: &Self, w: &mut W, task: &Task) -> Result<(), Error> {
        let title = if task.done {
            Style::new().strikethrough().paint(&task.title)
        } else {
            Style::default().paint(&task.title)
        };

        writeln!(
            w,
            "{:>4}. {} +{} {}",
            task.id,
            title,
            Style::default().paint(task.priority.to_string()),
            Style::default().paint(if self.is_smart_list {
                format!(":{}", task.list)
            } else {
                String::from("")
            }),
        )
    }
}
