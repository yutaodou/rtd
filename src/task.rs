extern crate time;

use self::time::Date;
use std::str::FromStr;
use time::OffsetDateTime;

pub const SMART_LISTS: &[&str; 1] = &["today"];

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub today: String,
    pub list: String,
    pub priority: Priority,
    pub created_at: OffsetDateTime,
    pub completed_at: Option<OffsetDateTime>,
    pub due_date: Option<Date>,
}

impl Task {
    pub fn new(title: String, list: String, priority: Priority, due_date: Option<Date>) -> Task {
        Task {
            id: 0,
            title,
            done: false,
            today: "".to_string(),
            list,
            priority,
            created_at: OffsetDateTime::now(),
            completed_at: None,
            due_date,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create(
        id: u32,
        title: String,
        done: bool,
        list: String,
        priority: Priority,
        today: String,
        created_at: OffsetDateTime,
        completed_at: Option<OffsetDateTime>,
        due_date: Option<Date>,
    ) -> Task {
        Task {
            id,
            title,
            done,
            today,
            list,
            priority,
            created_at,
            completed_at,
            due_date,
        }
    }

    pub fn mark_completed(self: &mut Self) {
        self.done = true;
        self.completed_at = Some(OffsetDateTime::now());
    }

    pub fn mark_uncompleted(self: &mut Self) {
        self.done = false;
        self.completed_at = None;
    }

    pub fn is_in_list(&self, list: &str) -> bool {
        match list.to_lowercase().as_str() {
            "today" => self.is_marked_for_today(),
            _ => self.list.to_lowercase().eq(list.to_lowercase().as_str()),
        }
    }

    pub fn mark_for_today(self: &mut Self) {
        self.today = Task::today();
    }

    pub fn remove_from_today(self: &mut Self) {
        self.today = String::new();
    }

    pub fn is_marked_for_today(&self) -> bool {
        self.today.eq(Task::today().as_str())
    }

    fn today() -> String {
        OffsetDateTime::now().format("%F")
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Priority {
    Low,
    High,
    Medium,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => String::from("low"),
            Priority::High => String::from("high"),
            Priority::Medium => String::from("medium"),
        }
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(priority: &str) -> Result<Self, Self::Err> {
        match priority.to_lowercase().as_str() {
            "l" | "low" => Ok(Priority::Low),
            "h" | "high" => Ok(Priority::High),
            "m" | "medium" => Ok(Priority::Medium),
            _ => Err(format!("Unsupported priority: {}", priority)),
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[cfg(test)]
pub mod test {
    use crate::task::{Priority, Task};
    use std::str::FromStr;

    #[test]
    fn test_parse_priority() {
        assert_eq!(Priority::from_str("l").unwrap(), Priority::Low);

        let result = Priority::from_str("haha").unwrap_err();
        assert_eq!("Unsupported priority: haha", result);
    }

    #[test]
    fn test_mark_for_today() {
        let mut task = Task::new(
            "test-todo".to_string(),
            "inbox".to_string(),
            Priority::High,
            None,
        );

        task.mark_for_today();
        assert_eq!(task.is_marked_for_today(), true);

        task.remove_from_today();
        assert_eq!(task.is_marked_for_today(), false);
    }

    #[test]
    fn test_in_list() {
        let mut task = Task::new(
            "test-todo".to_string(),
            "inbox".to_string(),
            Priority::High,
            None,
        );

        task.mark_for_today();
        assert_eq!(task.is_in_list("Today"), true);
        assert_eq!(task.is_in_list("Inbox"), true);

        task.remove_from_today();
        assert_eq!(task.is_in_list("today"), false);
    }

    #[test]
    fn test_mark_completed() {
        let mut task = Task::new(
            "test-todo".to_string(),
            "inbox".to_string(),
            Priority::High,
            None,
        );

        assert!(task.completed_at.is_none());

        task.mark_completed();
        assert!(task.done);
        assert!(task.completed_at.is_some());

        task.mark_uncompleted();
        assert_eq!(task.done, false);
        assert_eq!(task.completed_at.is_some(), false);
    }
}
