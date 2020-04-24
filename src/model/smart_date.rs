use std::ops::{Add, Deref};
use std::str::FromStr;
use time::{Date, Duration, OffsetDateTime, Weekday};

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SmartDate {
    date: Date,
}

impl FromStr for SmartDate {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        SmartDate::new(input, || OffsetDateTime::now_local().date())
    }
}

impl Deref for SmartDate {
    type Target = Date;

    fn deref(&self) -> &Self::Target {
        &self.date
    }
}

impl Display for SmartDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.date.format("%F").as_str())?;
        Ok(())
    }
}

impl SmartDate {
    fn new<F>(input: &str, now: F) -> Result<SmartDate, String>
    where
        F: Fn() -> Date,
    {
        Date::parse(input, "%F")
            .or_else(|_| Date::parse(input, "%-Y%m%d"))
            .map(|date| SmartDate { date })
            .or_else(|_| SmartDate::parse(input, now))
    }

    fn parse<F>(input: &str, now: F) -> Result<SmartDate, String>
    where
        F: Fn() -> Date,
    {
        let today = now();
        match input.to_lowercase().as_str() {
            "mon" | "monday" => Some(Weekday::Monday),
            "tue" | "tues" | "tuesday" => Some(Weekday::Tuesday),
            "wed" | "wednesday" => Some(Weekday::Wednesday),
            "thu" | "thur" | "thursday" => Some(Weekday::Thursday),
            "fri" | "friday" => Some(Weekday::Friday),
            "sat" | "saturday" => Some(Weekday::Saturday),
            "sun" | "sunday" => Some(Weekday::Sunday),
            "today" => Some(today.weekday()),
            "tomorrow" => Some(today.weekday().next()),
            _ => None,
        }
        .map(|due_date_weekday| {
            let today_weekday = today.weekday();

            let diff_days = due_date_weekday.number_days_from_monday() as i64
                - today_weekday.number_days_from_monday() as i64;

            let duration_offset = if diff_days >= 0 {
                Duration::days(diff_days)
            } else {
                Duration::days(diff_days + 7)
            };

            SmartDate {
                date: today.add(duration_offset),
            }
        })
        .ok_or_else(|| format!("I have no idea about: '{}'", input))
    }
}

#[cfg(test)]
mod test {
    use crate::model::SmartDate;
    use time::Date;

    const NOW: fn() -> Date = || Date::parse("2020-04-24", "%F").unwrap();

    #[test]
    fn test_smart_date() {
        let smart_date = SmartDate::new("today", NOW).unwrap();
        assert_eq!(smart_date.date.as_ymd(), (2020, 4, 24));

        let smart_date = SmartDate::new("tomorrow", NOW).unwrap();
        assert_eq!(smart_date.date.as_ymd(), (2020, 4, 25));
    }

    #[test]
    fn test_smart_date_in_this_week() {
        let smart_date = SmartDate::new("sat", NOW).unwrap();
        assert_eq!(smart_date.date.as_ymd(), (2020, 4, 25));
        let smart_date = SmartDate::new("Friday", NOW).unwrap();
        assert_eq!(smart_date.date.as_ymd(), (2020, 4, 24));
    }

    #[test]
    fn test_smart_date_in_next_week() {
        let next_monday = SmartDate::new("monday", NOW).unwrap();
        assert_eq!(next_monday.date.as_ymd(), (2020, 4, 27));

        let next_monday = SmartDate::new("Mon", NOW).unwrap();
        assert_eq!(next_monday.date.as_ymd(), (2020, 4, 27));
    }

    #[test]
    fn test_smart_date_iso_format() {
        let smart_date = SmartDate::new("2020-02-02", NOW).unwrap();
        assert_eq!(smart_date.date.as_ymd(), (2020, 2, 2));

        let smart_date = SmartDate::new("20200202", NOW).unwrap();
        assert_eq!(smart_date.date.as_ymd(), (2020, 2, 2));
    }

    #[test]
    fn test_invalid_smart_date() {
        let error = SmartDate::new("hello rust", NOW).unwrap_err();
        assert_eq!(error, "I have no idea about: 'hello rust'");
    }
}
