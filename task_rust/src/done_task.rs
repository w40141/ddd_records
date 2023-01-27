use chrono::{DateTime, Local};

use crate::task::Task;

#[derive(Debug)]
struct DoneTask {
    id: i64,
    title: String,
    due_date: DateTime<Local>,
}

impl Task for DoneTask {
    fn get_id() -> i64 {
        todo!()
    }

    fn get_title() -> String {
        todo!()
    }
}
