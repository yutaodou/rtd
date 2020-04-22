extern crate dirs;

use std::fs::DirBuilder;
use std::result::Result;

use rusqlite::{params, Connection, Row, NO_PARAMS};

use crate::task::Task;

pub fn open_connection() -> Connection {
    let database_file_path = super::database_file_path();
    let database_file_dir = database_file_path.parent().unwrap();
    if !database_file_dir.exists() {
        DirBuilder::new().create(database_file_dir).unwrap();
    }
    Connection::open(&database_file_path).unwrap()
}

pub fn update(task: &Task) -> Result<&Task, String> {
    let completed_at = match task.completed_at {
        Some(date) => date.timestamp(),
        None => 0,
    };

    let conn = open_connection();
    conn.execute_named(
        "UPDATE todo SET title = :title, list = :list, done = :done, today = :today, priority = :priority, completed_at = :completed_at WHERE id = :id",
        &[
            (":title", &task.title),
            (":list", &task.list.as_str()),
            (":priority", &task.priority.to_string()),
            (":done", if task.done { &1 } else { &0 }),
            (":today", &task.today),
            (":id", &task.id),
            (":completed_at", &completed_at),
        ]).unwrap();
    Ok(task)
}

pub fn add(task: &Task) -> Result<Task, String> {
    let conn = open_connection();
    conn.execute_named(
        "INSERT INTO todo (title, list, priority, created_at) VALUES (:title, :list, :priority, :created_at)",
        &[
            (":title", &task.title),
            (":list", &task.list.as_str()),
            (":priority", &task.priority.to_string()),
            (":created_at", &task.created_at.timestamp()),
        ],
    )
        .unwrap();

    let result = conn
        .prepare("SELECT * FROM todo ORDER BY id DESC LIMIT 1")
        .and_then(|mut stmt| stmt.query_row(NO_PARAMS, |row| Ok(map_to_task(row))));

    match result {
        Ok(task) => Ok(task),
        Err(err) => {
            eprintln!("{}", err.to_string());
            Err(String::from("Failed to add task"))
        }
    }
}

pub fn get(task_id: u32) -> Result<Task, String> {
    let conn = open_connection();
    let mut stmt = conn.prepare("SELECT * FROM todo WHERE id = (?1)").unwrap();
    let todo = stmt.query_row(params![task_id], |row| Ok(map_to_task(row)));

    match todo {
        Ok(task) => Ok(task),
        _ => Err(format!("Task with id '{}' not found", task_id)),
    }
}

pub fn get_all() -> Result<Vec<Task>, String> {
    let conn = open_connection();
    let mut stmt = conn.prepare("SELECT * FROM todo").unwrap();
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
        row.get("id").unwrap(),
        row.get("title").unwrap(),
        row.get("done").unwrap(),
        row.get("list").unwrap(),
        row.get("priority").unwrap(),
        row.get("today").unwrap(),
        row.get("created_at").unwrap(),
        row.get("completed_at").unwrap(),
    )
}
