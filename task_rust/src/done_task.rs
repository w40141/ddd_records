use chrono::{DateTime, Local};

use crate::task::Task;

#[derive(Debug)]
pub struct DoneTask {
    id: i64,
    title: String,
    done_date: DateTime<Local>,
}

impl DoneTask {
    pub fn new(id: i64, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            done_date: Local::now(),
        }
    }

    fn done_date(&self) -> &DateTime<Local> {
        &self.done_date
    }
}

impl Task for DoneTask {
    fn id(&self) -> &i64 {
        &self.id
    }

    fn title(&self) -> &String {
        &self.title
    }
}
