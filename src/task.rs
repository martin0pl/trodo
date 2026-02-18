pub struct Task {
    done: bool,
    title: String
}

impl Task {
    pub fn new_t(title: String) -> Task {
        Self {
            done: false,
            title : title,
        }
    }

    pub fn preparation_affichage(&self) -> String {
        if self.done {
            return(format!("[x] {}",self.title));
        }
        else {
            return(format!("[ ] {}",self.title));
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
}
