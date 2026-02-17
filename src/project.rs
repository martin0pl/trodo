use crate::Task;

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
}
