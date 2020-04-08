#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub list: String,
}

impl Task {
    pub fn new(title: String, list: String) -> Task {
        Task {
            id: 0,
            title,
            done: false,
            list,
        }
    }

    pub fn create(id: u32, title: String, done: u32, list: String) -> Task {
        Task {
            id,
            title,
            done: done == 1,
            list,
        }
    }
}
