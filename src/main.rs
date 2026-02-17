mod task;
use task::Task;

fn main() {

    let t1: Task = Task::new_t("Première tâche".to_string());

    t1.afficher();

}
