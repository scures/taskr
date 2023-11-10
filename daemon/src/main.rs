use common::Config;
use daemonize::Daemonize;
use task_exec::check_and_execute_tasks;
mod task_exec;

fn main() {
    let config = Config::new().unwrap();
    let stdout = std::fs::File::create(format!("{}/std.out", config.os_path)).unwrap();
    let stderr = std::fs::File::create(format!("{}/std.err", config.os_path)).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(format!("{}/taskr.pid", config.os_path))
        .stdout(stdout)
        .stderr(stderr)
        .working_directory("/tmp/taskr") // for default behavior.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => check_and_execute_tasks(),
        Err(e) => eprintln!("Error, {}", e),
    }
}
