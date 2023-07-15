use chrono::{Duration, NaiveDateTime, Utc};

/// Utilities for manipulating tasks.
use super::context::Context;
use serde_with::{serde_as, DurationSeconds};

const DEFAULT_DURATION_MIN: i64 = 25;

/// A description of a thing to do.
#[serde_as]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Task {
    pub name: String,
    description: String,
    pub priority: i32,
    done: bool,
    #[serde_as(as = "DurationSeconds<i64>")]
    duration: Duration,
    context: Option<String>,
    created: NaiveDateTime,
}

impl Task {
    pub fn new(
        name: String,
        description: String,
        priority: i32,
        done: bool,
        context: Option<String>,
    ) -> Task {
        Task {
            name,
            description,
            priority,
            done,
            duration: Duration::minutes(DEFAULT_DURATION_MIN),
            context,
            created: Utc::now().naive_utc(),
        }
    }

    pub fn new_with_duration(
        name: String,
        description: String,
        priority: i32,
        done: bool,
        duration: Duration,
        context: Option<String>,
    ) -> Task {
        Task {
            name,
            description,
            priority,
            done,
            duration,
            context,
            created: Utc::now().naive_utc(),
        }
    }

    pub fn filter_context_tasks(context: &Context, tasks: Vec<Task>) -> Vec<Task> {
        let mut filtered_tasks: Vec<Task> = vec![];

        for task in tasks {
            if task.context.is_some()
                && !task.done
                && (task.clone().context.unwrap().to_lowercase() == context.name.to_lowercase())
            {
                filtered_tasks.push(task);
            }
        }

        filtered_tasks
    }

    pub fn do_work(&mut self, duration: Duration) {
        if self.duration < duration {
            self.duration = Duration::minutes(0);
        } else {
            self.duration = self.duration - duration;
        }
    }

    pub fn has_work_remaining(&mut self) -> bool {
        return self.duration > Duration::minutes(0);
    }
}
