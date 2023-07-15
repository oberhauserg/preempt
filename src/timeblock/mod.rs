/// Various forms of scheduling.
use chrono::{Duration, NaiveDate, NaiveTime, Weekday};

use super::location::GeoFence;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};

/// A concrete block of time. Used for immovable/unschedulable schedule items and scheduler outputs.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeBlock {
    pub name: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl TimeBlock {
    pub fn new(
        start_time: NaiveTime,
        end_time: NaiveTime,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> TimeBlock {
        TimeBlock {
            name: None,
            start_time,
            end_time,
            start_date,
            end_date,
        }
    }

    pub fn new_named(
        name: String,
        start_time: NaiveTime,
        end_time: NaiveTime,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> TimeBlock {
        TimeBlock {
            name: Some(name),
            start_time,
            end_time,
            start_date,
            end_date,
        }
    }

    /// TODO: Just say everything intersects for now.
    pub fn intersects(&self, other: TimeBlock) -> bool {
        true
    }
}

/// A (potentially recurring) event with fuzzy planning.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FuzzyTimeBlock {
    start_time: NaiveTime,
    #[serde_as(as = "DurationSeconds<i64>")]
    start_uncertainty: Duration,
    end_time: NaiveTime,
    #[serde_as(as = "DurationSeconds<i64>")]
    end_uncertainty: Duration,
    place: Option<GeoFence>,
    weekdays: Option<Vec<Weekday>>,
}
