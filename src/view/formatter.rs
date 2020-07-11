use ansi_term::Colour::Red;
use ansi_term::{ANSIGenericString, Style};
use std::string::ToString;
use crate::model::Task;

pub struct Formatter<'a> {
    task: &'a Task
}

impl  <'a> Formatter<'a> {
    pub fn new(task: &Task) -> Self {
        return Formatter { task }
    }

    pub fn done(self) -> String {
        if self.task.done {
            String::from("✔")
        } else {
            String::from("")
        }
    }

    pub fn task_id(self) -> String {
        format!("{}.", self.task.id)
    }

    pub fn priority(self) -> String {
        format!("+{}", self.task.priority.to_string())
    }

    pub fn title(self) -> String {
        format!("{}", self.task.title)
    }

    pub fn task_list(self, show_list: bool) -> String {
        if show_list {
            format!(":{}", self.task.list)
        }else {
            ""
        }
   }

    fn due_date(self) -> ANSIGenericString<String>{
        let due_date = self.task
            .due_date
            .as_ref()
            .map_or_else(|| "".to_string(), |due_date| format!("@{}", due_date));

        if self.task.is_overdue() {
            Style::default().fg(Red)
        } else {
            Style::default()
        }
            .paint(due_date)
    }
}