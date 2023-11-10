# Taskr
<!-- Import image from /assets/trim and center it -->
<p align="center">
  <img src="https://raw.githubusercontent.com/scures/taskr/master/assets/trim.png" alt="Taskr logo" width="200px" />
</p>
<p>Small project in Rust used to schedule tasks for later execution, outputting the result to a Discord channel.</p>


> ❗️ **NOTE**: This project is still in development and is not ready for production use.


## 🧑🏻‍🔬 Use cases
1. **Automated Reporting**: You can schedule tasks that generate reports. This can be useful for teams that use Discord for communication and want to have regular updates on certain metrics.
2. **Scheduled System Maintenance**: You can use Taskr to schedule system maintenance tasks such as cleaning up temporary files or restarting services.
3. **Scheduled Backups**: You can use Taskr to schedule backups of your system or database.
4. **Use GH Actions to schedule a task**: Some times you want to schedule a task to be executed in a remote machine. You can use GH Actions to schedule a task in a remote machine using Taskr.

## ⚙️ How it works
The program is split in two parts:
- the daemon (taskr-daemon) which is responsible for executing the tasks
- the client (taskr) which is responsible for scheduling tasks

> ❗️ **NOTE**: A better CLI is coming soon. This is temporary.

### CLI
```bash
taskr "docker run -d redis" 10 # this will schedule the command `docker run -d redis` to be executed in 10 seconds.

taskr kill # this will kill the daemon
```

## 📋 TODO
> List of things I want to cover and improve in the future. If you want to contribute, feel free to open a PR/Issue.

#### Generic

- [ ] Finish the install.sh script and explore using [trust](https://github.com/japaric/trust) to build the project.
- [ ] Handle Discord URL secret in a better way (env variable?)

#### CLI:
- [x] Handle command & arguments
- [ ] Allow to list executed tasks and retrieve their output
- [ ] Use Clap instead of building our own command handler

#### Daemon:
- [x] Spawn detached process
- [ ] Report command `stdout` and `stderr` to Hooks (Discord, Slack, etc)
- [ ] While reporting to discord, make sure it won't exceed the limit of Discord's allowed message size


#### Nice to have
Things that would be nice to have but are not a priority.
- [ ] Allow to add timeout in natural language using [event-parser](https://github.com/isaacrlee/event-parser)
- [ ] Add unit & integration tests
