use crate::model::Task;
use std::string::ToString;

pub struct Formatter<'a> {
    task: &'a Task,
}

impl<'a> Formatter<'a> {
    pub fn new(task: &'a Task) -> Self {
        Formatter { task }
    }

    pub fn done(&self) -> String {
        if self.task.done {
            String::from("✔")
        } else {
            String::from("")
        }
    }

    pub fn task_id(&self) -> String {
        format!("{}.", self.task.id)
    }

    pub fn priority(&self) -> String {
        format!("+{}", self.task.priority.to_string())
    }

    pub fn title(&self, width: Option<usize>) -> String {
        format!("{:width$}", self.task.title, width = width.unwrap_or(0))
    }

    pub fn task_list(&self, show_list: bool) -> String {
        if show_list {
            format!(":{}", self.task.list)
        } else {
            "".to_string()
        }
    }

    pub fn due_date(&self) -> String {
        self.task
            .due_date
            .as_ref()
            .map_or_else(|| "".to_string(), |due_date| format!("@{}", due_date))
    }
}
