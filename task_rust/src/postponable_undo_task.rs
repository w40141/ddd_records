use crate::task::Task;
use crate::undo_task::UndoneTask;
use chrono::{DateTime, Local};

#[derive(Debug)]
struct PostponableUndoTask {
    id: i64,
    title: String,
    due_date: DateTime<Local>,
    postpone_count: i64,
}

impl Task for PostponableUndoTask {
    fn get_id() -> i64 {
        todo!()
    }

    fn get_title() -> String {
        todo!()
    }
    // add code here
}

impl PostponableUndoTask {
    fn postpone(&self) -> PostponableUndoTask {
        return PostponableUndoTask {
            id: self.id,
            title: self.title.to_owned(),
            due_date: self.due_date,
            postpone_count: self.postpone_count + 1,
        };
    }
}

impl UndoneTask for PostponableUndoTask {
    fn get_due_date() {
        todo!()
    }

    fn done() {
        todo!()
    }
}
