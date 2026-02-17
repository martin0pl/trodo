pub struct Task {
    title: String
}

impl Task {
    pub fn new_t(title: String) -> Task {
        Self {
            title : title,
        }
    }
}
