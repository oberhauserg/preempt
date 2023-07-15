use std::cmp::Ordering;

/// Utilities for manipulating context.
use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};

use super::timeblock::TimeBlock;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContextException {
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
    #[serde_as(as = "DurationSeconds<i64>")]
    transition_time: Duration,
}

impl ContextException {
    pub fn new(
        date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
        transition_time: Duration,
    ) -> Self {
        ContextException {
            date,
            start_time,
            end_time,
            transition_time,
        }
    }
}

/// A description of a context. A context is described sort-of like a recurring calendar invite.
///
/// Note that contexts do not have a timezone. Timezones are applied right before outputting
/// results and are not included on this level to keep things simple. All times and dates are assumed to be UTC.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    pub name: String,
    pub days: Vec<Weekday>,
    pub start: NaiveTime,
    pub end: NaiveTime,
    #[serde_as(as = "DurationSeconds<i64>")]
    pub transition: Duration,
    exceptions: Vec<ContextException>,
}

impl Context {
    pub fn new(
        name: &str,
        days: Vec<Weekday>,
        start: NaiveTime,
        end: NaiveTime,
        transition: Duration,
    ) -> Self {
        Self {
            name: name.to_string(),
            days,
            start,
            end,
            transition,
            exceptions: vec![],
        }
    }

    fn get_days(&self) -> Vec<Weekday> {
        let mut days = self.days.clone();

        days.sort_unstable_by(|a, b| {
            if a.number_from_monday() - 1 > b.number_from_monday() - 1 {
                return Ordering::Greater;
            } else if a.number_from_monday() - 1 < b.number_from_monday() - 1 {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        });

        return days;
    }

    pub fn get_timeblock(&self, day: NaiveDate) -> Option<TimeBlock> {
        if self.days.contains(&day.weekday()) {
            Some(TimeBlock::new(self.start, self.end, day, day))
        } else {
            None
        }
    }

    pub fn print(&self) {
        println!("Context - {}", self.name);
        print!("- Days: ");

        if self.days.is_empty() {
            println!("None Set");
        } else {
            let days: Vec<String> = self.get_days().iter().map(|day| day.to_string()).collect();
            println!("{}", days.join(", "));
        }

        println!("- Start Time: {}", self.start.format("%H:%M"));
        println!("- End Time: {}", self.end.format("%H:%M"));

        let transition_minutes = self.transition.num_minutes();
        if transition_minutes >= 60 {
            println!(
                "- Transition Time: {:.2} hours",
                transition_minutes as f64 / 60.0
            );
        } else {
            println!("- Transition Time: {} minutes", transition_minutes);
        }

        if !self.exceptions.is_empty() {
            println!("- Exceptions:");
            for exception in &self.exceptions {
                println!(
                    "  * {}, {} to {}",
                    exception.date,
                    exception.start_time.format("%H:%M"),
                    exception.end_time.format("%H:%M")
                );
            }
        } else {
            println!("- Exceptions: None");
        }
    }
}
