use crate::task::Task;

pub trait UndoneTask: Task {
    fn get_due_date();
    fn done();
}
