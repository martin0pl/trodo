use std::fs;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::env;

use crate::Task;

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    current_project : i32,
    tasks : Vec<Task>,
}

impl App {
    pub fn new() -> App {
        Self {
            current_project : -1,
            tasks : Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);

        self.tasks.sort_by(|a, b| {
            match (a.get_due_date(), b.get_due_date()) {
                (Some(da), Some(db)) => da.cmp(&db),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }

    pub fn show_tasks(&self) {
        if self.tasks.len() > 0 {
            for i in 0..self.tasks.len() {
                println!("{} - {}",i,&self.tasks[i].preparation_affichage());
            }
        } else {
            println!("No tasks");
        }
    }

    pub fn show_today_tasks(&self) {
        let mut one_today_task = false;

        if self.tasks.len() > 0 {
            for i in 0..self.tasks.len() {
                if self.tasks[i].is_today() {
                    one_today_task = true;
                    println!("{} - {}", i, &self.tasks[i].preparation_affichage());
                }
            }
        }

        if self.tasks.len() > 0 && !one_today_task {
            println!("No tasks today");
        } else if self.tasks.len() == 0 {
            println!("No tasks");
        }
    }

    pub fn done_task(&mut self, num: usize) {
        self.tasks[num].done();
    }

    pub fn undone_task(&mut self, num: usize) {
        self.tasks[num].undone();
    }

    pub fn save(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self).expect("Error of serialization");
        fs::write(filename, json).expect("Impossible to reach the save file");
    }

    pub fn load_or_create(filename: &str) -> App {

        let home_dir = env::var("HOME").expect("Impossible to reach HOME directory");
        let full_path = format!("{}/trodo-save", home_dir);
        let path = Path::new(&full_path);

        // If the folder does not exist, we create it
        if !path.exists() {
            let _ = fs::create_dir(path);
        }

        // If the save file does not exist, we create it with an empty App
        if fs::metadata(filename).is_err() {
            let temp_app = App::new();
            temp_app.save(filename);
            return temp_app;
        }

        let json = fs::read_to_string(filename).expect("Reading error");
        serde_json::from_str(&json).expect("JSON format error")
    }

    pub fn delete_done (&mut self) {
        for i in (0..self.tasks.len()).rev() {
            if self.tasks[i].get_done() {
                self.tasks.remove(i);
            }
        }
    }

    pub fn delete_all (&mut self) {
        self.tasks = Vec::new();
    }

    pub fn delete_task (&mut self, num: usize) {
        self.tasks.remove(num);
    }

    pub fn get_nb_tasks (&self) -> usize {
        self.tasks.len()
    }
}
