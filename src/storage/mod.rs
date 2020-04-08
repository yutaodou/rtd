use rusqlite::{params, Connection, NO_PARAMS};

use crate::task::Task;

const DB_FILE_PATH: &str = "./todo";

pub fn init_database() {
    let conn = Connection::open(&DB_FILE_PATH).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo  (
            id              INTEGER PRIMARY KEY,
            title           TEXT NOT NULL,
            done            INTEGER DEFAULT 0,
            list            TEXT DEFAULT '',
            priority        TEXT DEFAULT 'medium'
            )",
        params![],
    )
    .unwrap();
}

pub fn add(task: &Task) -> Result<Task, &'static str> {
    init_database();
    let conn = Connection::open(&DB_FILE_PATH).unwrap();
    conn.execute(
        "INSERT INTO todo (title, list, priority) VALUES (?1, ?2, ?3)",
        params![task.title, task.list, task.priority.to_string()],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, title, done, list, priority FROM todo ORDER BY id DESC LIMIT 1")
        .unwrap();
    let todo = stmt
        .query_row(NO_PARAMS, |row| {
            Ok(Task::create(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .unwrap();

    Ok(todo)
}

pub fn done(task_id: u32) -> Result<Task, &'static str> {
    let conn = Connection::open(&DB_FILE_PATH).unwrap();
    conn.execute("UPDATE todo SET done = 1 WHERE id = (?1)", params![task_id])
        .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, title, done, list, priority FROM todo WHERE id = (?1)")
        .unwrap();
    let todo = stmt
        .query_row(params![task_id], |row| {
            Ok(Task::create(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .unwrap();

    Ok(todo)
}

pub fn get_all() -> Result<Vec<Task>, &'static str> {
    init_database();

    let conn = Connection::open(&DB_FILE_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT id, title, done, list, priority FROM todo")
        .unwrap();
    let todo_iter = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(Task::create(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .unwrap();

    let mut result: Vec<Task> = Vec::new();
    for task in todo_iter {
        result.push(task.unwrap());
    }
    Ok(result)
}
