use std::str::FromStr;

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
    use crate::model::Priority;
    use std::str::FromStr;

    #[test]
    fn test_parse_priority() {
        assert_eq!(Priority::from_str("l").unwrap(), Priority::Low);

        let result = Priority::from_str("haha").unwrap_err();
        assert_eq!("Unsupported priority: haha", result);
    }
}
