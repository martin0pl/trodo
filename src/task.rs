use serde::{Serialize, Deserialize};

use chrono::{DateTime, Utc, Duration};

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

        let mut result;

        let date_str = match &self.due_date {
            Some(date) => date.format("(%Y-%m-%d)").to_string(),
            None => "".to_string(),
        };

        if self.done {
            result = format!("[x] {} {}",self.title,date_str);
        }
        else {
            result = format!("[ ] {} {}",self.title,date_str);
        }

        if self.is_late() {
            result += " (late)";
        } else if self.is_today() {
            result += " (today)";
        } else if self.is_tomorrow() {
            result += " (tomorrow)";
        }

        result
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

    pub fn get_due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date
    }

    pub fn is_today(&self) -> bool {
        match self.due_date {
            Some(date) => date.date_naive() == Utc::now().date_naive(),
            None => false,
        }
    }

    pub fn is_late(&self) -> bool {
        match self.due_date {
            Some(date) => date.date_naive() < Utc::now().date_naive(),
            None => false,
        }
    }

    pub fn is_tomorrow(&self) -> bool {
        match self.due_date {
            Some(date) => date.date_naive() == Utc::now().date_naive() + Duration::days(1),
            None => false,
        }
    }
}
