#![allow(unused)]

mod task;
mod project;
mod app;

use task::Task;
use project::Project;
use app::App;

use std::env;
use chrono::{DateTime, Utc, TimeZone};

fn main() {

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
        // trodo new task "task name"
        else if args[0] == "new" && args[1] == "task" && args.len() == 3 {

            let title = args[2].clone();

            app.add_task(Task::new(title,None));

            app.save(&save_file);

            println!("Tâche ajoutée !");
        }
        // trodo new task "task name" "AAAA-MM-JJ"
        else if args[0] == "new" && args[1] == "task" {

            let title = args[2].clone();
            let mut due_date = None;

            let date_str = format!("{} 12:00:00", args[3]);
            if let Ok(naive_date) = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S") {
                due_date = Some(Utc.from_utc_datetime(&naive_date));
            }

            app.add_task(Task::new(title,due_date));

            app.save(&save_file);

            println!("Tâche ajoutée !");
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
            app.delete_all();

            app.save(&save_file);
        }
        // trodo delete task_num
        else if args[0] == "delete"{
            let indice = args[1].parse::<usize>().unwrap_or(0);
            app.delete_task(indice);

            app.save(&save_file);
        }
        else {
            println!("Commande non reconnue");
        }
    }
}
