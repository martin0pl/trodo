pub struct Task {
    title: String
}

impl Task {
    pub fn new_t(title: String) -> Task {
        Self {
            title : title,
        }
    }

    pub fn afficher(&self) {
        println!("Affichage de la tâche");
        println!("Titre : {}",self.titre);
    }
}
