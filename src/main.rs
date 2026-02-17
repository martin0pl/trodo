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

    let mut app: App = App::new();

    let t1: Task = Task::new_t("Première tâche".to_string());
    let t2: Task = Task::new_t("Deuxieme tâche".to_string());
    let t3: Task = Task::new_t("Troisieme tâche".to_string());

    app.add_task(t1);
    app.add_task(t2);
    app.add_task(t3);

}
