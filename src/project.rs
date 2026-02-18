use serde::{Serialize, Deserialize};

use crate::Task;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    title : String,
    tasks : Vec<Task>,
}

impl Project {
    pub fn new (title : String) -> Project {
        Self {
            title : title,
            tasks : Vec::new(),
        }
    }
    
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
