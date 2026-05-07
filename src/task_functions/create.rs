use crate::Task;
use std::fs::{self, File};
use std::io::prelude::*;

impl Task {
    /// Writes a task as a file with the contents given
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let task = Task::init(0, "example");
    /// task.write_file(0.to_string()); 
    /// ```
    pub fn write_file(&self, task_id: String) {
        let mut file: File = File::create(format!("tasks/task{}.txt", task_id)).unwrap();

        let id_string: String = format!("{}\n", self.id);
        file.write_all(id_string.as_bytes()).unwrap();

        let title_string = format!("{}\n", self.title);
        file.write_all(title_string.as_bytes()).unwrap();

        file.flush();
    }
}
