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
        if self.done {
            return format!("[x] {}",self.title);
        }
        else {
            return format!("[ ] {}",self.title);
        }
    }

    pub fn afficher(&self) {
        if self.done {
            println!("[x] {}",self.title);
        }
        else {
            println!("[ ] {}",self.title);
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
