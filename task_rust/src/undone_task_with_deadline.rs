use chrono::{DateTime, Local};

use crate::task::Task;

#[derive(Debug)]
struct UndoneTaskWithDeadline {
    id: i64,
    title: String,
    due_date: DateTime<Local>,
}
