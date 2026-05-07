use crate::Task;
use std::fs::{self, File};

impl Task {
    /// Deletes a task
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// delete_file(0.to_string());
    /// ```
    pub fn delete_file(task_id: String) -> String {
        fs::remove_file(format!("tasks/task{}.txt", task_id));
        return task_id;
    }
}
