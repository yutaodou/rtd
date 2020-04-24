use clap::ArgMatches;
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use time::{Date, Duration, Weekday};

use crate::task::Priority;

#[derive(Debug)]
pub struct ToDoArgs {
    pub list: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub free_args: Vec<String>,
}

impl ToDoArgs {
    pub fn parse_priority(&self) -> Result<Option<Priority>, String> {
        match &self.priority {
            None => Ok(None),
            Some(input) => Priority::from_str(input.as_str()).map(Some),
        }
    }

    pub fn parse_due_date(self: &Self) -> Result<Option<Date>, String> {
        match &self.due_date {
            None => Ok(None),
            Some(input) => Date::parse(&input, "%F")
                .or_else(|_| Date::parse(&input, "%-Y%m%d"))
                .or_else(|_| ToDoArgs::smart_date(&input))
                .map(Some)
                .map_err(|_| format!("Invalid due date: {}", input)),
        }
    }

    fn smart_date(input: &str) -> Result<Date, String> {
        match input.to_lowercase().as_str() {
            "mon" | "monday" => Some(Weekday::Monday),
            "tue" | "tuesday" => Some(Weekday::Tuesday),
            "wed" | "wednesday" => Some(Weekday::Wednesday),
            "thu" | "thursday" => Some(Weekday::Thursday),
            "fri" | "friday" => Some(Weekday::Friday),
            "sat" | "saturday" => Some(Weekday::Saturday),
            "sun" | "sunday" => Some(Weekday::Sunday),
            "today" => Some(OffsetDateTime::now_local().weekday()),
            "tomorrow" => Some(OffsetDateTime::now_local().weekday().next()),
            _ => None,
        }
        .map(|due_date_weekday| {
            let today = OffsetDateTime::now_local().date();
            let today_weekday = today.weekday();

            let diff_days = due_date_weekday.number_days_from_monday() as i64
                - today_weekday.number_days_from_monday() as i64;

            let duration_offset = if diff_days >= 0 {
                Duration::days(diff_days)
            } else {
                Duration::days(diff_days + 7)
            };

            OffsetDateTime::now_local().add(duration_offset).date()
        })
        .ok_or_else(|| format!("Unknown due date: {}", input))
    }

    pub fn parse(args: &ArgMatches) -> ToDoArgs {
        let extract = |arg: &str| arg.get(1..arg.len()).map(|a| a.to_string());

        let mut list = None;
        let mut priority = None;
        let mut due_date = None;
        let mut free_args = vec![];

        args.values_of("INPUT").unwrap().for_each(|arg| {
            if arg.starts_with(':') && arg.len() > 1 {
                list = extract(arg);
            } else if arg.starts_with('+') && arg.len() > 1 {
                priority = extract(arg);
            } else if arg.starts_with('@') && arg.len() > 1 {
                due_date = extract(arg);
            } else {
                free_args.push(arg.to_string());
            }
        });

        ToDoArgs {
            list,
            priority,
            due_date,
            free_args,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::todo_args::ToDoArgs;
    use std::ops::Add;
    use time::{Duration, OffsetDateTime};

    #[test]
    fn test_parse_due_date() {
        let mut add = ToDoArgs {
            list: None,
            due_date: Some("20200202".to_string()),
            priority: None,
            free_args: vec![],
        };
        assert_eq!(
            add.parse_due_date().unwrap(),
            Some(time::date!(2020 - 02 - 02))
        );

        add.due_date = Some("2020-02-02".to_string());
        assert_eq!(
            add.parse_due_date().unwrap(),
            Some(time::date!(2020 - 02 - 02))
        );

        add.due_date = Some("2020".to_string());
        assert_eq!(
            add.parse_due_date().unwrap_err(),
            "Invalid due date: 2020".to_string()
        );

        add.due_date = None;
        assert_eq!(add.parse_due_date().unwrap(), None);

        let tomorrow = OffsetDateTime::now_local().add(Duration::days(1));
        add.due_date = Some("Tomorrow".to_string());
        assert_eq!(add.parse_due_date().unwrap(), Some(tomorrow.date()));

        add.due_date = Some("today".to_string());
        assert_eq!(
            add.parse_due_date().unwrap(),
            Some(OffsetDateTime::now_local().date())
        );
    }
}
