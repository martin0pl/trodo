//#![allow(unused)]

mod task;
mod project;
mod app;

use task::Task;
use project::Project;
use app::App;

use std::env;
use chrono::{DateTime, Utc, TimeZone, TimeDelta};
use std::io;

fn main() {

    const VERSION:&str = "2026-03-01";

    let home_dir = env::var("HOME").expect("Impossible to reach HOME directory");
    let save_file = format!("{}/trodo-save/save_trodo.json", home_dir);

    let mut app = App::load_or_create(&save_file);

    let args: Vec<String> = env::args().collect();

    // Vérification de s'il y a une commande
    if args.len() == 1
    {
        app.show_tasks();
    }
    else
    {
        // On ne prend pas le premier argument qui est le nom du programme
        let args: Vec<String> = args[1..].to_vec();

        // Gestion des commandes

        // trodo list
        if args[0] == "list" {
            app.show_tasks();
        }
        // trodo version
        else if args[0] == "version" {
            println!("Version : {}",VERSION);
        }
        // trodo info
        else if args[0] == "info" {
            println!("Trodo");
            println!("Developper : martin0pl");
            println!("Programming language : Rust");
            println!("Version : {}",VERSION);
            println!("Github repository : https://github.com/martin0pl/trodo");
        }
        // trodo current project
        else if args[0] == "current" && args[1] == "project" {
            let num_current_project = app.get_current_project();
            if num_current_project != -1 {
                println!("----- Current project : ");
                println!("Project number : {}",num_current_project);
                println!("Project name : {}",app.get_project_title(num_current_project));
            } else {
                println!("No current project");
            }
        }
        // trodo new task "task name"
        else if args[0] == "new" && args[1] == "task" && args.len() == 3 {

            let title = args[2].clone();

            app.add_task(Task::new(title,None));

            app.save(&save_file);

            println!("Task added !");
        }
        // trodo new task "task name" today
        else if args[0] == "new" && args[1] == "task" && args[3] == "today" {
            let title = args[2].clone();
            let due_date = Some(Utc::now());

            app.add_task(Task::new(title,due_date));

            app.save(&save_file);

            println!("Task added !");
        }
        // trodo new task "task name" tomorrow
        else if args[0] == "new" && args[1] == "task" && args[3] == "tomorrow" {
            let title = args[2].clone();
            let due_date = Some(Utc::now() + TimeDelta::try_days(1).expect("Invalid duration"));

            app.add_task(Task::new(title,due_date));

            app.save(&save_file);

            println!("Task added !");
        }
        // trodo new task "task name" "YYYY-MM-DD"
        else if args[0] == "new" && args[1] == "task" {

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
        else if args[0] == "done"{
            let indice = args[1].parse::<usize>().unwrap_or(0);
            app.done_task(indice);

            app.save(&save_file);
        }
        // trodo undone task_num
        else if args[0] == "undone"{
            let indice = args[1].parse::<usize>().unwrap_or(0);
            app.undone_task(indice);

            app.save(&save_file);
        }
        // trodo delete done
        else if args[0] == "delete" && args[1] == "done" {
            app.delete_done();

            app.save(&save_file);
        }
        // trodo delete all
        else if args[0] == "delete" && args[1] == "all" {

            println!("Are you sur to delete all your tasks ? y for yes or n for no");

            let mut confirmation: String = String::new();
            io::stdin().read_line(&mut confirmation);

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
        else if args[0] == "delete"{
            let indice = args[1].parse::<usize>().unwrap_or(0);
            app.delete_task(indice);

            app.save(&save_file);
        }
        else {
            println!("Command not recognized");
        }
    }
}
