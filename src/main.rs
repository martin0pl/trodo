#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod task;
mod project;
mod app;

use task::Task;
use project::Project;
use app::App;

fn main() {
    
    let app: App = App::new();

    let t1: Task = Task::new_t("Première tâche".to_string());

    t1.afficher();

}
