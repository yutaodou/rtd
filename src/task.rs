extern crate time;

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub today: String,
    pub list: String,
    pub priority: Priority,
}

impl Task {
    pub fn new(title: String, list: String, priority: Priority) -> Task {
        Task {
            id: 0,
            title,
            done: false,
            today: "".to_string(),
            list,
            priority,
        }
    }

    pub fn create(
        id: u32,
        title: String,
        done: u32,
        list: String,
        priority: String,
        today: String,
    ) -> Task {
        Task {
            id,
            title,
            done: done == 1,
            today,
            list,
            priority: Priority::from(&priority).unwrap(),
        }
    }

    pub fn mark_for_today(self: Self) {}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Priority {
    Low,
    High,
    Medium,
}

impl Priority {
    pub fn from(priority: &str) -> Result<Priority, &'static str> {
        match priority.to_lowercase().as_str() {
            "l" | "low" => Ok(Priority::Low),
            "h" | "high" => Ok(Priority::High),
            "m" | "medium" => Ok(Priority::Medium),
            _ => Err("Unsupported priority"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Priority::Low => String::from("low"),
            Priority::High => String::from("high"),
            Priority::Medium => String::from("medium"),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::time::OffsetDateTime;
    use crate::task::Priority;

    #[test]
    fn test_priority() {
        assert_eq!(Priority::from("l").unwrap(), Priority::Low);
    }

    #[test]
    fn test_today() {
        let now = OffsetDateTime::now().timestamp() / 86400;
        assert_eq!(now, 123);
    }
}
