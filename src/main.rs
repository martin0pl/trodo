mod task;
mod project;
mod app;

use task::Task;
use project::Project;
use app::App;

use std::env;
use chrono::{Utc, TimeZone, TimeDelta};
use std::io;

fn main() {

    const VERSION:&str = "2026-03-04";

    let home_dir = env::var("HOME").expect("Impossible to reach HOME directory");
    let save_file = format!("{}/trodo-save/save_trodo.json", home_dir);

    let mut app = App::load_or_create(&save_file);

    let args: Vec<String> = env::args().collect();

    // If there is no command
    if args.len() == 1
    {
        app.show_tasks();
    }
    else
    {
        // Ignore the first argument which is the name of the program
        let args: Vec<String> = args[1..].to_vec();

        // Commands

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
        // trodo projects
        else if args[0] == "projects" {
            app.show_all_projects();
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
        // trodo close current project
        else if args[0] == "close" && args[1] == "current" && args[2] == "project" {
            app.set_current_project(-1);

            println!("Current project closed");
        }
        // trodo open project num_project
        else if args[0] == "open" && args[1] == "project" {

            let indice = args[2].parse().expect("Unvalid indice");

            if indice as usize <= app.get_nb_projects() {
                app.set_current_project(indice);

                app.save(&save_file);

                println!("Current project set to {}", args[2]);
            } else {
                println!("Unvalid indice");
            }
        }
        // trodo new project "project name"
        else if args[0] == "new" && args[1] == "project" && args.len() == 3 {
            let project = Project::new(args[2].clone());

            app.add_project(project);

            app.save(&save_file);

            println!("Project added !");
        }
        // trodo delete project num_project
        else if args[0] == "delete" && args[1] == "project"  && args.len() == 3 {

            let indice = args[1].parse::<usize>().unwrap_or(0);
            
            app.delete_project(indice);

            app.save(&save_file);

            println!("Project deleted");

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

            if indice < app.get_nb_tasks() {
                app.done_task(indice);

                app.save(&save_file);
            }
        }
        // trodo undone task_num
        else if args[0] == "undone"{
            let indice = args[1].parse::<usize>().unwrap_or(0);

            if indice < app.get_nb_tasks() {
                app.undone_task(indice);

                app.save(&save_file);
            }
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
        else if args[0] == "delete"{
            let indice = args[1].parse::<usize>().unwrap_or(0);

            if indice < app.get_nb_tasks() {
                app.delete_task(indice);

                app.save(&save_file);
            }
        }
        else {
            println!("Command not recognized");
        }
    }
}
