use common::{kill_daemon_pid, parse_scheduler, write_task, Config, Task};
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let conf = Config::new().unwrap();
    let args: Vec<_> = std::env::args().collect();
    let construct_command = &args[1].split(" ").collect::<Vec<&str>>();

    if construct_command[0].to_string() == "kill".to_string() {
        kill_daemon_pid();

        return Ok(());
    }

    // TODO: Move to Clap at some point.
    if args.len() > 2 {
        let command_args = construct_command[1..]
            .to_vec()
            .iter()
            .map(|x| x.to_string())
            .filter(|x| !x.is_empty())
            .collect::<Vec<String>>();

        let new_task = Task {
            time: parse_scheduler(args[2].parse::<i64>().unwrap()).expect("Failed to parse time"),
            command: construct_command[0].to_string(),
            args: command_args,
            id: Uuid::new_v4(),
        };

        write_task(&conf.tasks_file_name, new_task);

        Ok(())
    } else {
        println!("❗️One arguments are missing ∫ taskr \"command arg1 arg2 arg3\" 10");
        Ok(())
    }
}
