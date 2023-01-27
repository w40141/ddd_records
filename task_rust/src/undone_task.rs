use chrono::{DateTime, Local};

use crate::{task::Task, done_task::DoneTask};

pub trait UndoneTask: Task {
    fn due_date(&self) -> &DateTime<Local>;
    fn done(&self) -> DoneTask;
}
