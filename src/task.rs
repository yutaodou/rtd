#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub list: String,
    pub priority: Priority,
}

impl Task {
    pub fn new(title: String, list: String) -> Task {
        Task {
            id: 0,
            title,
            done: false,
            list,
            priority: Priority::Medium,
        }
    }

    pub fn create(id: u32, title: String, done: u32, list: String, priority: String) -> Task {
        Task {
            id,
            title,
            done: done == 1,
            list,
            priority: Priority::from(&priority).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
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
    use crate::task::Priority;

    #[test]
    fn test_priority() {
        assert_eq!(Priority::from("l").unwrap(), Priority::Low);
    }
}
