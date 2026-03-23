# Pampa
Simple package that allows to use basic CRUD operations, such as creating or reading tasks.

# Important Note:
As this is the first version of this package, it isn't polished and minor details can be found, such as having to create a "tasks" folder in the project directory.

## Structure
```
project
    - src
        - main.rs
    - tasks
        - task0.txt
        - task1.txt
        - ...
    - Cargo.toml

```

# Example

```rust
use pampa::Task;

// Initialize Task
let task_id = 1;
let task_title: String = "Walk the dog";
let task = Task::init(task_id, task_title);

// Create the file for task.
task.write_file(task_id.to_string());

// Read task
let read = Task::read_file(task_id.parse().unwrap());
println!("{}", read)

// Read all tasks
let all_files = Task::read_all_files();
for file in all_files {
    println!("{}", file)
}
// Delete Task
let delete = Task::delete_file(task_id.to_string());

```
