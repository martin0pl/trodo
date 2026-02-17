mod task;
mod project;

use task::Task;
use project::Project;

fn main() {

    let t1: Task = Task::new_t("Première tâche".to_string());

    t1.afficher();

}
