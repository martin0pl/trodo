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

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    
    pub fn preparation_affichage(&self) -> String {
        self.title.clone()
    }
    
    pub fn show(&self, num_project: usize) {
        println!("Title of the project : {}",self.title);
        
        if self.tasks.len() == 0 {
            println!("No tasks in the project");
        } 
        else {
            for i in 0..self.tasks.len() {
                println!("{}-{} {}",num_project,i,self.tasks[i].preparation_affichage());
            }
        }
    }
    
    pub fn sort(&mut self) {
        self.tasks.sort_by(|a, b| {
            match (a.get_due_date(), b.get_due_date()) {
                (Some(da), Some(db)) => da.cmp(&db),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }
}