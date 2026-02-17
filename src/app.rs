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
}
