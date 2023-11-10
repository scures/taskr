use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{Config, Task};

// For now, we only handle minutes, we want to use something like https://github.com/isaacrlee/event-parser for natural language to date time.
pub fn parse_scheduler(input: i64) -> Option<chrono::DateTime<Utc>> {
    let seconds = Duration::seconds(input);

    Some(Utc::now() + seconds)
}

// Reads from .JSON file and returns a vector of Tasks
pub fn read_tasks(file_name: &String) -> Vec<Task> {
    let contents =
        std::fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap();

    tasks
}

// Writes a task to .JSON file
pub fn write_task(file_name: &String, task: Task) {
    let mut tasks = read_tasks(file_name);

    tasks.push(task);

    let string_task = serde_json::to_string(&tasks).unwrap();

    match std::fs::write(file_name, string_task) {
        Ok(_) => println!("Task added successfully"),
        Err(e) => println!("Failed to add task: {}", e),
    }
}

// Removes a task from JSON file
pub fn remove_task(task: Task) {
    let conf = crate::conf::Config::new().unwrap();
    let tasks = read_tasks(&conf.tasks_file_name);
    let filtered_tasks: Vec<Task> = tasks.into_iter().filter(|x| x.id != task.id).collect();
    let string_task = serde_json::to_string(&filtered_tasks).unwrap();

    std::fs::write(&conf.tasks_file_name, string_task).expect("Unable to write file");

    write_task(&conf.completed_tasks_file_name, task);
}

pub fn write_stdout_logs(tasks: Task, logs_path: &String, logs: String) {
    std::fs::write(&format!("{}/{}.txt", logs_path, tasks.id), logs).expect("Unable to write file");
}

// INFO: This is only for demo purposes, we should remove this.
pub fn write_demo_tasks() {
    let conf = crate::conf::Config::new().unwrap();
    let demo_tasks = vec![
        Task {
            time: parse_scheduler(1).expect("Failed to parse time"),
            command: "ls".to_string(),
            args: vec!["-ls".to_string()],
            id: Uuid::new_v4(),
        },
        Task {
            time: parse_scheduler(1).expect("Failed to parse time"),
            command: "docker".to_string(),
            args: vec!["ps".to_string(), "--format".to_string(), "json".to_string()],
            id: Uuid::new_v4(),
        },
    ];
    let string_task = serde_json::to_string(&demo_tasks).unwrap();

    std::fs::write(&conf.tasks_file_name, string_task).expect("Unable to write file");

    remove_logs(&conf.logs_dir)
}

pub fn remove_logs(logs_dir: &String) {
    let files = std::fs::read_dir(logs_dir).unwrap();

    for path in files {
        std::fs::remove_file(path.unwrap().path()).unwrap();
    }
}

pub fn ensure_exists(path: &str, is_dir: bool) {
    let path_obj = std::path::Path::new(path);

    if !path_obj.exists() {
        if is_dir {
            std::fs::create_dir_all(path_obj)
                .expect(format!("Unable to create dir: {}", path).as_str());
        } else {
            std::fs::write(path_obj, "[]")
                .expect(format!("Unable to write file: {}", path).as_str());
        }
    }
}

pub fn kill_daemon_pid() -> Option<String> {
    let config = Config::new().unwrap();

    // Extracts PID from file
    let pid_file = format!("{}/taskr.pid", config.os_path);
    let pid: String = std::fs::read_to_string(&pid_file).expect("Failed to read pid file");
    let pid: i32 = pid.trim().parse().expect("Failed to parse pid");

    println!("Killing process with pid: {}", pid);
    let kill = unsafe { libc::kill(pid, libc::SIGTERM) };

    std::fs::remove_file(&pid_file).expect("Failed to remove pid file");
    println!("Kill result: {}", kill);

    Some(format!("Kill result: {}", kill).to_string())
}
