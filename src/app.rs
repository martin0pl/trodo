use std::fs;
use serde::{Serialize, Deserialize};

use crate::Task;
use crate::Project;

#[derive(Serialize, Deserialize, Debug)]
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
        for i in 0..self.tasks.len() {
            println!("{} - {}",i,&self.tasks[i].preparation_affichage());
        }
    }

    pub fn done_task(&mut self, num: usize) {
        self.tasks[num].done();
    }
    
    pub fn undone_task(&mut self, num: usize) {
        self.tasks[num].undone();
    }

    pub fn save(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self).expect("Erreur de sérialisation");
        fs::write(filename, json).expect("Impossible d'écrire le fichier");
    }

    pub fn load_or_create(filename: &str) -> App {
        // Si le fichier n'existe pas, on en crée un avec une App vide
        if fs::metadata(filename).is_err() {
            let temp_app = App::new();
            temp_app.save(filename);
            return temp_app;
        }

        let json = fs::read_to_string(filename).expect("Erreur lecture");
        serde_json::from_str(&json).expect("Erreur format JSON")
    }
}
