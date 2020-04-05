use crate::task::Task;

use rusqlite::{params, Connection};

const DB_FILE_PATH: &str = "./todo";

pub fn add(task: &Task) -> Result<(), &'static str> {
    let conn = Connection::open(&DB_FILE_PATH).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo  (
                  id              INTEGER PRIMARY KEY,
                  title           TEXT NOT NULL
                  )",
        params![],
    )
    .unwrap();

    conn.execute("INSERT INTO todo (title) VALUES (?1)", params![task.title])
        .unwrap();
    Ok(())
}

pub fn get_all() -> Result<Vec<Task>, &'static str> {
    let conn = Connection::open(&DB_FILE_PATH).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo  (
                  id              INTEGER PRIMARY KEY,
                  title           TEXT NOT NULL
                  )",
        params![],
    )
    .unwrap();
    let mut stmt = conn.prepare("SELECT id, title FROM todo").unwrap();
    let todo_iter = stmt
        .query_map(params![], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })
        .unwrap();

    let mut result: Vec<Task> = Vec::new();
    for task in todo_iter {
        result.push(task.unwrap());
    }
    Ok(result)
}
