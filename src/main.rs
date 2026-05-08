mod task;
mod app;

use task::Task;
use app::App;

use std::env;
use chrono::{Utc, TimeZone, TimeDelta, DateTime};
use std::io;

fn main() {

    const VERSION:&str = "0.9.0";

    let home_dir = env::var("HOME").expect("Impossible to reach HOME directory");
    let save_file = format!("{}/trodo-save/save_trodo.json", home_dir);

    let mut app = App::load_or_create(&save_file);

    let args: Vec<String> = env::args().skip(1).collect();

    // If there is no command
    if args.len() == 0
    {
        println!("Trodo");
        println!("Developper : martin0pl");
        println!("Programming language : Rust");
        println!("Version : {}",VERSION);
        println!("Github repository : https://github.com/martin0pl/trodo");
    }
    else
    {
        // Commands

        // trodo list
        if args.len() == 1 && args[0] == "list" {
            app.show_tasks();
        }
        // trodo version
        else if args.len() == 1 && args[0] == "version" {
            println!("Version : {}",VERSION);
        }
        // trodo info
        else if args.len() == 1 && args[0] == "info" {
            println!("Trodo");
            println!("Developper : martin0pl");
            println!("Programming language : Rust");
            println!("Version : {}",VERSION);
            println!("Github repository : https://github.com/martin0pl/trodo");
        }
        // trodo today
        else if args.len() == 1 && args[0] == "today" {
            app.show_today_and_late_tasks();
        }
        // trodo soon
        else if args.len() == 1 && args[0] == "soon" {
            app.show_soon_tasks();
        }
        // trodo late
        else if args.len() == 1 && args[0] == "late" {
            app.show_late_tasks();
        }
        // trodo tomorrow
        else if args.len() == 1 && args[0] == "tomorrow" {
            app.show_tomorrow_tasks();
        }
        // trodo help
        else if args.len() == 1 && args[0] == "help" {
            println!("Trodo Commands:");

            println!("General:");
            println!("  trodo                                     : Show all tasks without project");
            println!("  trodo info                                : Show all info");
            println!("  trodo version                             : Show the current version");
            println!("  trodo help                                : Show this help message\n");

            println!("Task Display:");
            println!("  trodo list                                : Show all tasks without project");
            println!("  trodo list YYYY-MM-DD                     : Show tasks at the specify date");
            println!("  trodo today                               : Show the today's tasks");
            println!("  trodo tomorrow                            : Show the tomorrow's tasks");
            println!("  trodo soon                                : Show the tasks in the next 7 days");
            println!("  trodo late                                : Show late tasks\n");

            println!("Task Creation:");
            println!("  trodo new task \"task name\"                : Add a new task");
            println!("  trodo new task \"task name\" \"YYYY-MM-DD\"   : Add a new task with due date");
            println!("  trodo new task \"task name\" today          : Add a task to do today");
            println!("  trodo new task \"task name\" tomorrow       : Add a task to do tomorrow\n");

            println!("Task Management:");
            println!("  trodo done task_num                       : Mark a task as done");
            println!("  trodo undone task_num                     : Mark a task as undone");
            println!("  trodo delay task_num YYYY-MM-DD           : Delay a task to a day\n");

            println!("Deletion:");
            println!("  trodo delete task_num                     : Delete the task");
            println!("  trodo delete done                         : Delete all the done tasks");
            println!("  trodo delete all                          : Delete all the tasks");
        }
        // trodo list YYYY-MM-DD
        else if args.len() == 2 && args[0] == "list" {
            let date = args[1].clone();
            app.show_tasks_for_date(&date);
        }
        // trodo delay task_num YYYY-MM-DD
        else if args.len() == 3 && args[0] == "delay" {
            let task_id = args[1].parse::<usize>().unwrap();

            let mut date_str = args[2].clone();
            date_str.push_str(" 12:00:00");
            let mut date: Option<DateTime<Utc>> = None;
            if let Ok(naive_date) = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S") {
                date = Some(Utc.from_utc_datetime(&naive_date));
            }

            app.delay_task(task_id, date.expect("REASON"));

            app.save(&save_file);
            println!("Task delayed !");
        }
        // trodo new task "task name"
        else if args.len() == 3 && args[0] == "new" && args[1] == "task" {

            let title = args[2].clone();

            app.add_task(Task::new(title,None));

            app.save(&save_file);

            println!("Task added !");
        }
        // trodo new task "task name" today
        else if args.len() == 4 && args[0] == "new" && args[1] == "task" && args[3] == "today" {
            let title = args[2].clone();
            let due_date = Some(Utc::now());

            app.add_task(Task::new(title,due_date));

            app.save(&save_file);

            println!("Task added !");
        }
        // trodo new task "task name" tomorrow
        else if args.len() == 4 && args[0] == "new" && args[1] == "task" && args[3] == "tomorrow" {
            let title = args[2].clone();
            let due_date = Some(Utc::now() + TimeDelta::try_days(1).expect("Invalid duration"));

            app.add_task(Task::new(title,due_date));

            app.save(&save_file);

            println!("Task added !");
        }
        // trodo new task "task name" "YYYY-MM-DD"
        else if args.len() == 4 && args[0] == "new" && args[1] == "task" {

            let title = args[2].clone();
            let mut due_date = None;

            let date_str = format!("{} 12:00:00", args[3]);
            if let Ok(naive_date) = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S") {
                due_date = Some(Utc.from_utc_datetime(&naive_date));
            }

            app.add_task(Task::new(title,due_date));

            app.save(&save_file);

            println!("Task added !");
        }
        // trodo done task_num
        else if args.len() == 2 && args[0] == "done" {
            let indice = args[1].parse::<usize>().unwrap_or(0);

            if indice < app.get_nb_tasks() {
                app.done_task(indice);

                app.save(&save_file);
            }
        }
        // trodo undone task_num
        else if args.len() == 2 && args[0] == "undone" {
            let indice = args[1].parse::<usize>().unwrap_or(0);

            if indice < app.get_nb_tasks() {
                app.undone_task(indice);

                app.save(&save_file);
            }
        }
        // trodo delete done
        else if args.len() == 2 && args[0] == "delete" && args[1] == "done" {
            app.delete_done();

            app.save(&save_file);
        }
        // trodo delete all
        else if args.len() == 2 && args[0] == "delete" && args[1] == "all" {

            println!("Are you sur to delete all your tasks ? y for yes or n for no");

            let mut confirmation: String = String::new();
            let _ = io::stdin().read_line(&mut confirmation);

            if confirmation.ends_with('\n') {
                confirmation.pop();
            }

            if confirmation=="y" {
                app.delete_all();

                app.save(&save_file);

                println!("All tasks deleted");
            } else {
                println!("Command cancelled");
            }
        }
        // trodo delete task_num
        else if args.len() == 2 && args[0] == "delete" {
            let indice = args[1].parse::<usize>().unwrap_or(0);

            if indice < app.get_nb_tasks() {
                app.delete_task(indice);

                app.save(&save_file);
            }
        }
        else {
            println!("Command not recognized or invalid arguments.");
            println!("Type 'trodo help' to see the list of available commands.");
        }
    }
}
