use std::fs;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::env;

use crate::Task;
use crate::Project;

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    tasks : Vec<Task>,
    projects : Vec<Project>,
    version : String,
}

impl App {
    pub fn new() -> App {
        Self {
            tasks : Vec::new(),
            projects : Vec::new(),
            version : "2026-02-20".to_string(),
        }
    }
    
    pub fn show_version(&self) {
        println!("Version : {}",self.version)
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

        let home_dir = env::var("HOME").expect("Impossible to reach HOME directory");
        let full_path = format!("{}/trodo-save", home_dir);
        let path = Path::new(&full_path);

        // Si le dossier n'existe pas, on en crée un
        if !path.exists() {
            fs::create_dir(path);
        }

        // Si le fichier n'existe pas, on en crée un avec une App vide
        if fs::metadata(filename).is_err() {
            let temp_app = App::new();
            temp_app.save(filename);
            return temp_app;
        }

        let json = fs::read_to_string(filename).expect("Erreur lecture");
        serde_json::from_str(&json).expect("Erreur format JSON")
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

    pub fn delete_task (&mut self, num : usize) {
        self.tasks.remove(num);
    }
}
