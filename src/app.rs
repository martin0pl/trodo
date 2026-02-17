use crate::Task;
use crate::Project;

pub struct App {
    tasks : Vec<Task>,
    projects : Vec<Project>,
}

impl App {
    pub fn new() -> App {
        Self {
            tasks : Vec::new(),
            projects : Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn show_tasks(&self) {
        for task in &self.tasks {
            task.afficher();
        }
    }
}
