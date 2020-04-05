#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            id: 0,
            title: title,
        }
    }
}
