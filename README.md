# Trodo

An open-source project developed in Rust. Trodo is a terminal-based command-line interface (CLI) to manage your to-do list anywhere on your device, with persistent local storage.

## Installation

Download the binary file and put in a directory which is in your PATH

## Commands

### General

| Command                                     | Action                           |
| :-----------------------------------------: | :------------------------------: |
| `trodo`                                     | Show all info                    |
| `trodo info`                                | Show all info                    |
| `trodo version`                             | Show the current version         |
| `trodo help`                                | Show this help message           |

### Task Display

| Command                                     | Action                                  |
| :-----------------------------------------: | :-------------------------------------: |
| `trodo list`                                | Show all tasks without project          |
| `trodo list YYYY-MM-DD`                     | Show tasks at the specify date          |
| `trodo today`                               | Show the today's tasks                  |
| `trodo tomorrow`                            | Show the tomorrow's tasks               |
| `trodo soon`                                | Show the tasks in the next 7 days       |
| `trodo late`                                | Show late tasks                         |

### Task Creation

| Command                                     | Action                                  |
| :-----------------------------------------: | :-------------------------------------: |
| `trodo new task "task name"`                | Add a new task                          |
| `trodo new task "task name" "YYYY-MM-DD"`   | Add a new task with due date            |
| `trodo new task "task name" today`          | Add a task to do today                  |
| `trodo new task "task name" tomorrow`       | Add a task to do tomorrow               |

### Task Management

| Command                                     | Action                                  |
| :-----------------------------------------: | :-------------------------------------: |
| `trodo done task_num`                       | Mark a task as done                     |
| `trodo undone task_num`                     | Mark a task as undone                   |
| `trodo delay task_num YYYY-MM-DD`           | Delay a task to a day                   |

### Deletion

| Command                                     | Action                         |
| :-----------------------------------------: | :----------------------------: |
| `trodo delete task_num`                     | Delete the task                |
| `trodo delete done`                         | Delete all the done tasks      |
| `trodo delete all`                          | Delete all the tasks           |

## Roadmap

### Road to Version 2
- `trodo settings`
  - auto_list_on_changes : bool
  - auto_delete_on_done : bool
  - limit_show_tasks_number : i32
- `trodo` launch the script to do not use the prefix "trodo"
- `trodo export to markdown`

## Save file

The save file is save in a ~/trodo-save directory
