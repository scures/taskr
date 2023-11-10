use chrono::Utc;
use common::{notify_via_hook, read_tasks, remove_task, write_stdout_logs, Config, Task};

// TODO: Move this to detached process and run it in the background
// It should always run, maybe we need to read the task from file
pub fn check_and_execute_tasks() {
    let sleep_time = 3;
    let conf = Config::new().unwrap();

    println!(
        "😈 Taskr Daemon has started. Checking for new tasks every {}",
        sleep_time
    );

    loop {
        let tasks = read_tasks(&conf.tasks_file_name.to_string());
        let now = Utc::now();

        if tasks.len() >= 1 {
            for task in tasks {
                if now > task.time {
                    println!("Executing task: {:?}", task);

                    match task_exec(task.clone()) {
                        Some(x) => {
                            println!("Task executed successfully {}", x.to_string());

                            write_stdout_logs(task.clone(), &conf.logs_dir, x.to_string());

                            match notify_via_hook(&conf.webhook_url, task.clone(), &x.to_string()) {
                                Ok(_) => println!("Notification sent successfully"),
                                Err(e) => eprintln!("Failed to send notification. Error: {:?}", e),
                            }
                        }

                        None => println!("Failed to execute {}", task.command.to_string()),
                    }

                    // TODO: Notify vía Discord Hooks
                    remove_task(task.clone());
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(sleep_time));
    }
}

pub fn task_exec(task: Task) -> Option<String> {
    let command = std::process::Command::new(task.command)
        .args(task.args)
        .output();

    println!("🦀 Command, {:#?}", command);

    match command {
        Ok(x) => {
            let stdout = String::from_utf8(x.stdout).unwrap();
            let stderr = String::from_utf8(x.stderr).unwrap();

            if !stderr.is_empty() {
                Some(stderr)
            } else {
                Some(stdout)
            }
        }

        Err(e) => {
            eprintln!("Failed to execute command. Error: {:?}", e);
            None
        }
    }
}
