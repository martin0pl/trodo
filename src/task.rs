use serde::{Serialize, Deserialize};

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    done: bool,
    title: String,
    due_date : Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(title: String, due_date: Option<DateTime<Utc>>) -> Task {
        Self {
            done: false,
            title : title,
            due_date : due_date,
        }
    }

    pub fn preparation_affichage(&self) -> String {
        
        let date_str = match &self.due_date {
            Some(date) => date.format("(%Y-%m-%d)").to_string(),
            None => "".to_string(),
        };
        
        if self.done {            
            return format!("[x] {} {}",self.title,date_str);
        }
        else {
            return format!("[ ] {} {}",self.title,date_str);
        }
    }

    pub fn done(&mut self) {
        self.done = true;
    }

    pub fn undone(&mut self) {
        self.done = false;
    }

    pub fn get_done(&self) -> bool {
        return self.done;
    }
}
