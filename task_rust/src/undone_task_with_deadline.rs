use chrono::{DateTime, Local};

use crate::{done_task::DoneTask, task::Task, undone_task::UndoneTask};

#[derive(Debug)]
pub struct UndoneTaskWithDeadline {
    id: i64,
    title: String,
    due_date: DateTime<Local>,
}

impl UndoneTaskWithDeadline {
    pub fn new(id: i64, title: impl Into<String>, due_date: &DateTime<Local>) -> Self {
        Self {
            id,
            title: title.into(),
            due_date: due_date.clone(),
        }
    }
}

impl Task for UndoneTaskWithDeadline {
    fn id(&self) -> &i64 {
        &self.id
    }

    fn title(&self) -> &String {
        &self.title
    }
}

impl UndoneTask for UndoneTaskWithDeadline {
    fn due_date(&self) -> &DateTime<Local> {
        &self.due_date
    }

    fn done(&self) -> DoneTask {
        DoneTask::new(self.id, &self.title)
    }
}
