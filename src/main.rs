#![allow(unused)]

mod task;
mod project;
mod app;

use task::Task;
use project::Project;
use app::App;

use std::env;

fn main() {

    let mut app: App = App::new();

    // Zone des variables de test ----------------------------------

    let t1: Task = Task::new_t("Première tâche".to_string());
    let t2: Task = Task::new_t("Deuxieme tâche".to_string());
    let t3: Task = Task::new_t("Troisieme tâche".to_string());

    app.add_task(t1);
    app.add_task(t2);
    app.add_task(t3);

    // -------------------------------------------------------------

    let args: Vec<String> = env::args().collect();

    // Vérification de si il y a des arguments
    if args.len() == 1 {
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
        if args[0] == "new" && args[1] == "task" {
            if args.len() == 3 {
                app.add_task(Task::new_t(args[2].clone()));
                println!("Tâche <{}> ajoutée !",args[2]);
            }
        }
        else {
            println!("Commande non reconnue");
        }
    }
}
