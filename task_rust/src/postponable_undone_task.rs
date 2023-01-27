use chrono::{DateTime, Local};

use crate::done_task::DoneTask;
use crate::task::Task;
use crate::undone_task::UndoneTask;
use crate::undone_task_with_deadline::UndoneTaskWithDeadline;

#[derive(Debug)]
struct PostponableUndoneTask {
    id: i64,
    title: String,
    due_date: DateTime<Local>,
    postpone_count: i64,
}

impl Task for PostponableUndoneTask {
    fn id(&self) -> &i64 {
        &self.id
    }

    fn title(&self) -> &String {
        &self.title
    }
    // add code here
}

impl PostponableUndoneTask {
    fn postpone(&self) -> Box<dyn UndoneTask> {
        if self.postpone_count < 3 {
            return Box::new(PostponableUndoneTask {
                id: self.id,
                title: self.title.to_owned(),
                due_date: self.due_date,
                postpone_count: self.postpone_count + 1,
            })
        } else {
            return Box::new(UndoneTaskWithDeadline::new(
                self.id,
                self.title.to_owned(),
                &self.due_date,
            ))
        };
    }
}

impl UndoneTask for PostponableUndoneTask {
    fn due_date(&self) -> &DateTime<Local> {
        &self.due_date
    }

    fn done(&self) -> DoneTask {
        DoneTask::new(self.id, &self.title)
    }
}
