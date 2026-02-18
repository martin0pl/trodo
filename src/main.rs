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
        if args[0] == "list" {
            app.show_tasks();
        }
        else if args[0] == "new" && args[1] == "task" {
            app.add_task(Task::new_t(args[2].clone()));

            app.save(save_file);

            println!("Tâche ajoutée et sauvegardée !");
        }
        else if args[0] == "done"{
            let indice = args[1].parse::<usize>().unwrap_or(0);
            app.done_task(indice);

            app.save(save_file);
        }
        else if args[0] == "undone"{
            let indice = args[1].parse::<usize>().unwrap_or(0);
            app.undone_task(indice);

            app.save(save_file);
        }
        else if args[0] == "delete" && args[1] == "done" {
            app.delete_done();

            app.save(save_file);
        }
        else if args[0] == "delete" && args[1] == "all" {
            app.delete_all();

            app.save(save_file);
        }
        else if args[0] == "delete"{
            let indice = args[1].parse::<usize>().unwrap_or(0);
            app.delete_task(indice);

            app.save(save_file);
        }
        else {
            println!("Commande non reconnue");
        }
    }
}
