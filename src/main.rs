mod task;
mod project;
mod app;

use task::Task;
use project::Project;
use app::App;

fn main() {

    let t1: Task = Task::new_t("Première tâche".to_string());

    t1.afficher();

}
