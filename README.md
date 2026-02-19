# Trodo

An open-source project developed in Rust. Trodo is a terminal-based command-line interface (CLI) to manage your to-do list anywhere on your device, with persistent local storage.

## Commands

| Command                                   | Action                         |
| :---------------------------------------: | :----------------------------: |
| `trodo`                                   | Show all tasks without project |
| `trodo list`                              | Show all tasks without project |
| `trodo new task "task name"`              | Add a new task                 |
| `trodo new task "task name" "AAAA-MM-JJ"` | Add a new task with due date   |
| `trodo done task_num`                     | Mark a task as done            |
| `trodo undone task_num`                   | Mark a task as undone          |
| `trodo delete task_num`                   | Delete the task                |
| `trodo delete done`                       | Delete all the done tasks      |
| `trodo delete all`                        | Delete all the tasks           |
