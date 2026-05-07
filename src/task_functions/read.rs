use crate::Task;
use std::fs::{self, File};
use std::io::prelude::*;

impl Task {
    /// Reads a task, using id
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// read_file(1.to_string());
    /// ```
    pub fn read_file(task_id: String) -> String {
        let mut file = File::open(format!("tasks/task{}.txt", task_id)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        return contents;
    }

    /// Reads all tasks
    ///
    /// # Examples
    /// 
    /// ```rust,ignore
    /// read_all_files();
    /// ```
    pub fn read_all_files() -> Vec<String> {
        let mut vec: Vec<String> = vec![];

        for entry in fs::read_dir("tasks").unwrap() {
            let entry = entry.unwrap();
            let dir = entry.path();
            let mut file = File::open(dir).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            vec.push(contents);
        }
        return vec;
    }

}
