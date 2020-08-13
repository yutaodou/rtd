use time::OffsetDateTime;

use super::Priority;
use super::SmartDate;

pub const SMART_LISTS: [&str; 1] = ["today"];

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
    pub due_date: Option<SmartDate>,
}

impl Task {
    pub fn new(
        title: String,
        list: String,
        priority: Priority,
        due_date: Option<SmartDate>,
    ) -> Task {
        Task {
            id: 0,
            title,
            done: false,
            today: "".to_string(),
            list,
            priority,
            created_at: OffsetDateTime::now_utc(),
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
        due_date: Option<SmartDate>,
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
        self.completed_at = Some(OffsetDateTime::now_utc());
    }

    pub fn mark_uncompleted(self: &mut Self) {
        self.done = false;
        self.completed_at = None;
    }

    pub fn is_in_list(&self, list: &str) -> bool {
        match list.to_lowercase().as_str() {
            "today" => self.is_marked_for_today() || self.is_due_today(),
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

    pub fn is_due_today (&self) -> bool {
        self.due_date.as_ref().map(|due_date| due_date.is_today()).unwrap_or(false)
    }

    fn today() -> String {
        OffsetDateTime::now_utc().format("%F")
    }

    pub fn is_overdue(&self) -> bool {
        !self.done
            && self.due_date.as_ref().map_or_else(
            || false,
            |due_date| OffsetDateTime::now_local().date().gt(&due_date),
        )
    }
}

#[cfg(test)]
pub mod test {
    use crate::model::{Priority, Task};

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
