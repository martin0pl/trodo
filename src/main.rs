#![allow(unused)]

mod task;
mod project;
mod app;

use task::Task;
use project::Project;
use app::App;

use std::env;

fn main() {

    let save_file = "save_trodo.json";

    let mut app = App::load_or_create(save_file);

    let args: Vec<String> = env::args().collect();

    // Vérification de si il y a des arguments
    if args.len() == 1
    {
        app.show_tasks();
    }
    else
    {
        // On ne prend pas le premier argument qui est le nom du programme
        let args: Vec<String> = args[1..].to_vec();

        // Gestion des commandes
        if args[0] == "list" {
            app.show_tasks();
        }
        else if args[0] == "new" && args[1] == "task" {
            app.add_task(Task::new_t(args[2].clone()));
            
            app.save(save_file);
            
            println!("Tâche ajoutée et enregistrée !");
        }
        else {
            println!("Commande non reconnue");
        }
    }
}
