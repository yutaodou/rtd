#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            id: 0,
            title: title,
            done: false
        }
    }

    pub fn create(id: u32, title: String, done: u32) -> Task {
        Task {
            id, title, done: done == 1
        }
    }
}
