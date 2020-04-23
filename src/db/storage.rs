extern crate dirs;
extern crate time;

use std::fs::DirBuilder;
use std::result::Result;
use std::str::FromStr;

use rusqlite::{params, Connection, Row, NO_PARAMS};

use self::time::{Date, OffsetDateTime};
use crate::task::{Task, Priority};

fn open_connection() -> Connection {
    let database_file_path = super::database_file_path();
    let database_file_dir = database_file_path.parent().unwrap();
    if !database_file_dir.exists() {
        DirBuilder::new().create(database_file_dir).unwrap();
    }
    Connection::open(&database_file_path).unwrap()
}

pub fn update(task: &Task) -> Result<&Task, String> {
    let conn = open_connection();
    conn.execute_named(
        "UPDATE todo SET title = :title, list = :list, done = :done, today = :today, priority = :priority, completed_at = :completed_at, due_date = :due_date WHERE id = :id",
        &[
            (":title", &task.title),
            (":list", &task.list.as_str()),
            (":priority", &task.priority.to_string()),
            (":done", if task.done { &1 } else { &0 }),
            (":today", &task.today),
            (":id", &task.id),
            (":completed_at", &task.completed_at.map_or(0, |date| date.timestamp())),
            (":due_date", &task.due_date.map_or("".to_string(), |date| date.format("%F"))),
        ]).unwrap();
    Ok(task)
}

pub fn add(task: &Task) -> Result<Task, String> {
    let conn = open_connection();
    conn.execute_named(
        "INSERT INTO todo (title, list, priority, due_date, created_at) VALUES (:title, :list, :priority, :due_date, :created_at)",
        &[
            (":title", &task.title),
            (":list", &task.list.as_str()),
            (":priority", &task.priority.to_string()),
            (":created_at", &task.created_at.timestamp()),
            (":due_date", &task.due_date.map_or("".to_string(), |date| date.format("%F"))),
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
    let due_date = row.get("due_date").ok()
        .map(|due_date: String| {
            if due_date.is_empty() {
                None
            } else {
                Date::parse(due_date, "%F").ok()
            }
        })
        .flatten();


    let completed_time = row.get("completed_at").ok()
        .map(|completed_at| if completed_at == 0 {
            None
        } else {
            Some(OffsetDateTime::from_unix_timestamp(completed_at))
        })
        .flatten();

    Task::create(
        row.get("id").unwrap(),
        row.get("title").unwrap(),
        row.get("done").ok().map_or(false, |done: u8| done == 1),
        row.get("list").unwrap(),
        row.get("priority").ok().map_or(Priority::Medium, |priority: String| Priority::from_str(&priority).unwrap()),
        row.get("today").unwrap(),
        row.get("created_at").ok().map(|created_at: i64| OffsetDateTime::from_unix_timestamp(created_at)).unwrap(),
        completed_time,
        due_date,
    )
}
