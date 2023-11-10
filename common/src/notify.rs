use serde_json::json;

use crate::Task;

pub fn notify_via_hook(webhook_url: &str, task: Task, message: &str) -> Result<(), reqwest::Error> {
    let payload = json!({
        "embeds": [
            {
                "title": format!("Task {} executed", task.command),
                "description": message,
                "color": 15258703,
            }
        ]
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(webhook_url).json(&payload).send()?;

    if res.status().is_success() {
        let body = res.text()?;
        println!("Response Text: {}", body);
    } else {
        println!("Request failed with status: {}", res.status());
    }

    Ok(())
}
