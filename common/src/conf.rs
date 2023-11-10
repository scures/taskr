use crate::utils::ensure_exists;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tasks_file_name: String,
    pub completed_tasks_file_name: String,
    pub logs_dir: String,
    pub os_path: String,
    pub webhook_url: String,
}

impl Config {
    pub fn new() -> Option<Config> {
        let os_path = match std::env::var("TMPDIR") {
            // Ok(val) => val.to_string(), // Uncomment for production
            Ok(_) => "/tmp/taskr".to_string(),
            Err(_e) => "/tmp/".to_string(),
        };

        let discord_hook = match std::fs::read_to_string(format!("{}/discord_webhook", &os_path)) {
            Ok(val) => val.to_string(),
            Err(_e) => {
                println!("Failed to read discord hook file, using default {}", _e);

                "foo".to_string()
            }
        };

        let config = Config {
            tasks_file_name: format!("{}/tasks.json", os_path),
            completed_tasks_file_name: format!("{}/completed_tasks.json", os_path),
            logs_dir: format!("{}/logs", os_path),
            os_path: os_path.to_string(),
            webhook_url: discord_hook,
        };

        // Prepares folders & files
        ensure_exists(&os_path, true);
        ensure_exists(&config.logs_dir, true);
        ensure_exists(&config.tasks_file_name, false);
        ensure_exists(&config.completed_tasks_file_name, false);

        Some(config)
    }
}
