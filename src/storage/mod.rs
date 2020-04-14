extern crate dirs;

use std::fs::DirBuilder;
use std::path::PathBuf;
use std::result::Result;

use rusqlite::{params, Connection, Row, NO_PARAMS};

use crate::task::Task;

fn database_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".rtd");
    path.push("database");
    path.set_extension("data");
    path
}

fn init_database() {
    let conn = open_connection();
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

fn open_connection() -> Connection {
    let database_file_path = database_file_path();
    let database_file_dir = database_file_path.parent().unwrap();
    if !database_file_dir.exists() {
        DirBuilder::new().create(database_file_dir).unwrap();
    }
    Connection::open(&database_file_path).unwrap()
}

pub fn add(task: &Task) -> Result<Task, &'static str> {
    init_database();
    let conn = open_connection();
    conn.execute(
        "INSERT INTO todo (title, list, priority) VALUES (?1, ?2, ?3)",
        params![task.title, task.list, task.priority.to_string()],
    )
    .unwrap();

    let result = conn
        .prepare("SELECT id, title, done, list, priority FROM todo ORDER BY id DESC LIMIT 1")
        .and_then(|mut stmt| stmt.query_row(NO_PARAMS, |row| Ok(map_to_task(row))));

    match result {
        Ok(task) => Ok(task),
        Err(err) => {
            eprintln!("{}", err.to_string());
            Err("Failed to add task")
        }
    }
}

pub fn done(task_id: u32, done: bool) -> Result<Task, String> {
    let completed = if done { 1 } else { 0 };
    let conn = open_connection();
    conn.execute(
        "UPDATE todo SET done = (?1) WHERE id = (?2)",
        params![completed, task_id],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, title, done, list, priority FROM todo WHERE id = (?1)")
        .unwrap();
    let todo = stmt.query_row(params![task_id], |row| Ok(map_to_task(row)));

    match todo {
        Ok(task) => Ok(task),
        _ => Err(format!("Task with id '{}' not found", task_id)),
    }
}

pub fn get_all() -> Result<Vec<Task>, &'static str> {
    init_database();

    let conn = open_connection();
    let mut stmt = conn
        .prepare("SELECT id, title, done, list, priority FROM todo")
        .unwrap();
    let todo_iter = stmt
        .query_map(NO_PARAMS, |row| Ok(map_to_task(row)))
        .unwrap();

    let mut result: Vec<Task> = Vec::new();
    for task in todo_iter {
        result.push(task.unwrap());
    }
    Ok(result)
}

fn map_to_task(row: &Row) -> Task {
    Task::create(
        row.get(0).unwrap(),
        row.get(1).unwrap(),
        row.get(2).unwrap(),
        row.get(3).unwrap(),
        row.get(4).unwrap(),
    )
}
