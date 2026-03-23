use std::fs::{self, File};
use std::io::prelude::*;

/// Struct made for creating tasks
/// Note: Create a folder named "tasks"
///
/// # Example
/// ```rust,ignore
/// Task {
///     id: 23,
///     title: "Get food"
/// }
/// ```
#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: u8,
    pub title: String,
}

impl Task {
    /// Initializes the Task
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let task: Task = Task::init(5, "walk dog".to_string());
    /// let expected = Task { id: 5, title: "walk dog".to_string() }
    /// assert_eq!(task, expected);
    /// ```
    pub fn init(id: u8, title: String) -> Self {
        Self {
            id: id,
            title: title,
        }
    }

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

    /// Reads a task, using id
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// read_file(1);
    /// ```
    pub fn read_file(task_id: i32) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup() {
        let _ = fs::create_dir_all("tasks");
    }
    fn teardown() {
        let _ = fs::remove_dir_all("tasks");
    }

    #[test]
    fn test_init() {
        let result = Task::init(0, "task".to_string());
        let expected = Task { id: 0, title: "task".to_string() };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_write_and_read_file() {
        setup();
        let task = Task::init(1, "tarea ejemplo".to_string());
        let task_id = "1".to_string();

        // Probar escritura
        task.write_file(task_id.clone());

        // Probar lectura
        let content = Task::read_file(1);
        let expected_content = "1\ntarea ejemplo\n";
        
        assert_eq!(content, expected_content);
        teardown();
    }

    #[test]
    fn test_delete_file() {
        setup();
        let task = Task::init(2, "tarea eliminar".to_string());
        let task_id = "2".to_string();

        task.write_file(task_id.clone());
        
        // Verificar que el archivo existe antes de borrar
        assert!(std::path::Path::new("tasks/task2.txt").exists());

        // Ejecutar borrado
        let deleted_id = Task::delete_file(task_id);
        
        assert_eq!(deleted_id, "2");
        assert!(!std::path::Path::new("tasks/task2.txt").exists());
        teardown();
    }

    #[test]
    fn test_read_all_files() {
        setup();
        let task1 = Task::init(1, "tarea 1".to_string());
        let task2 = Task::init(2, "tarea 2".to_string());

        task1.write_file("1".to_string());
        task2.write_file("2".to_string());

        let all_tasks = Task::read_all_files();

        assert_eq!(all_tasks.len(), 2);
        assert!(all_tasks.contains(&"1\ntarea 1\n".to_string()));
        assert!(all_tasks.contains(&"2\ntarea 2\n".to_string()));
        
        teardown();
    }
}
