use std::collections::VecDeque;

use chrono::{Duration, NaiveDate, NaiveTime};

/// Various forms of scheduling.
use super::context::Context;
use super::task::Task;
use super::timeblock::TimeBlock;

/// The priority class provides a simple way to implement Multilevel Queue Scheduling.
#[derive(Eq, PartialEq)]
enum PriorityClass {
    High = 2,
    Medium = 1,
    Low = 0,
}

fn get_task_priority(task: &Task) -> PriorityClass {
    if task.priority >= 10 {
        PriorityClass::High
    } else if task.priority <= 6 && task.priority >= 3 {
        PriorityClass::Medium
    } else {
        PriorityClass::Low
    }
}

fn get_priority_queue(tasks: &Vec<Task>, class: PriorityClass) -> VecDeque<Task> {
    let mut queue: VecDeque<Task> = VecDeque::new();

    for task in tasks {
        if get_task_priority(task) == class {
            queue.push_front(task.clone());
        }
    }

    queue
}

/// Creates
///
fn create_pomodoro_block(task: &Task, start_time: NaiveTime, date: NaiveDate) -> TimeBlock {
    TimeBlock::new_named(
        format!("Task - {}", task.name),
        start_time,
        start_time + Duration::minutes(25),
        date,
        date,
    )
}

fn create_pomodoro_rest(start_time: NaiveTime, date: NaiveDate, duration: Duration) -> TimeBlock {
    TimeBlock::new_named(
        format!("Break ({} minutes)", duration.num_minutes().to_string()),
        start_time,
        start_time + Duration::minutes(25),
        date,
        date,
    )
}

fn handle_task(
    queue: &mut VecDeque<Task>,
    cur_time: NaiveTime,
    schedule_date: NaiveDate,
    populated_time_block: &mut Vec<TimeBlock>,
) {
    if let Some(mut task) = queue.pop_back() {
        populated_time_block.push(create_pomodoro_block(&task, cur_time, schedule_date));
        task.do_work(Duration::minutes(25));
        if task.has_work_remaining() {
            queue.push_front(task);
        }
    }
}

/// This is the main scheduling logic.
///
/// The scheduler uses Multilevel Queue Scheduling strategy.
/// Which splits tasks into 3 priority levels - high, medium, and low.
///
///
/// 1. High Priority Queue: This queue contains tasks that are of high priority
/// and need to be performed as soon as possible. Tasks in this queue are
/// scheduled via a Shortest Job First (SJF) strategy. Tie breaking is done via
/// First Come First Served (FIFO).
///
/// 2. Medium Priority Queue: The medium priority queue is scheduled after the
/// medium priority queue is exhausted. It uses the same SJF and FIFO scheme as
/// the high priority queue.
///
/// 3. Low Priority Queue: The low priority queue is scheduled once the High
/// and Medium queues have been exhausted. To avoid starvation, a low
/// priority task is forcibly scheduled after 4 high or medium priority tasks
/// have been scheduled. Under normal circumstances, low priority tasks are
/// scheduled with a Round-Robin scheduling algorithm that employs a 25
/// minute time quanta.
///
/// With this scheduling stack up, a full pomodoro cycle is allowed to
/// finish with high and medium priority tasks before moving to lower
/// priority tasks.
///
fn populate_time_block(tasks: Vec<Task>, schedule_block: TimeBlock) -> Vec<TimeBlock> {
    let mut populated_time_block = Vec::new();
    let mut high_med_prio_tasks = 0;
    let mut total_tasks = 0;
    let mut forced_low_pri = false;

    const FORCED_LOW_PRIO_TASK: i32 = 4;
    let mut time_block_full = false;

    let mut high_priority_queue = get_priority_queue(&tasks, PriorityClass::High);
    let mut med_priority_queue = get_priority_queue(&tasks, PriorityClass::Medium);
    let mut low_priority_queue = get_priority_queue(&tasks, PriorityClass::Low);

    let mut cur_time: NaiveTime = schedule_block.start_time;

    while !time_block_full {
        if !high_priority_queue.is_empty() || !med_priority_queue.is_empty() {
            // Force inject low priority task if necessary
            if high_med_prio_tasks >= 1 && high_med_prio_tasks % FORCED_LOW_PRIO_TASK == 0 && !forced_low_pri {
                handle_task(
                    &mut low_priority_queue,
                    cur_time,
                    schedule_block.start_date,
                    &mut populated_time_block,
                );

                forced_low_pri = true;
            }
            else {
                if !high_priority_queue.is_empty() {
                    handle_task(
                        &mut high_priority_queue,
                        cur_time,
                        schedule_block.start_date,
                        &mut populated_time_block,
                    );
                    high_med_prio_tasks += 1;
                } else if !med_priority_queue.is_empty() {
                    handle_task(
                        &mut med_priority_queue,
                        cur_time,
                        schedule_block.start_date,
                        &mut populated_time_block,
                    );
                    high_med_prio_tasks += 1;
                }                

                forced_low_pri = false;
            }

            cur_time += Duration::minutes(25);
        } else if !low_priority_queue.is_empty() {
            handle_task(
                &mut low_priority_queue,
                cur_time,
                schedule_block.start_date,
                &mut populated_time_block,
            );
            cur_time += Duration::minutes(25);
        } else {
            // No tasks left!!
            time_block_full = true;
        }

        total_tasks += 1;

        if cur_time >= schedule_block.end_time {
            time_block_full = true;
        } else if !time_block_full {

            let rest_duration = if total_tasks % 4 == 0 {
                Duration::minutes(20)
            } else {
                Duration::minutes(5)
            };
            populated_time_block.push(create_pomodoro_rest(
                cur_time,
                schedule_block.start_date,
                rest_duration,
            ));
            cur_time += rest_duration;
        }
    }

    populated_time_block
}

/// This function builds a schedule for a single day.
/// TODO: Do more than one day.
pub fn build_schedule(
    contexts: &Vec<Context>,
    tasks: &Vec<Task>,
    schedule_block: TimeBlock,
) -> Vec<TimeBlock> {
    let mut schedule: Vec<TimeBlock> = vec![];

    // First, find which contexts are active during this time block.
    let mut active_contexts: Vec<Context> = vec![];

    for context in contexts {
        match context.get_timeblock(schedule_block.start_date) {
            Some(timeblock) => {
                schedule.append(&mut populate_time_block(
                    Task::filter_context_tasks(context, tasks.clone()),
                    timeblock,
                ));
            }
            None => {}
        }
    }

    schedule
}

pub fn print_schedule(schedule: Vec<TimeBlock>) {
    for block in schedule {
        println!(
            "{start} - {end} | {block_name}",
            start = block.start_time.to_string(),
            end = block.end_time.to_string(),
            block_name = match block.name {
                Some(name) => {
                    name
                }
                None => {
                    "Unnamed item".to_string()
                }
            }
        )
    }
}
