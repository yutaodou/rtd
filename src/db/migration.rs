use crate::db::storage;
use anyhow::{Error, Result};
use rusqlite::params;
use rusqlite::Connection;
use std::fs::DirBuilder;

pub fn migrate() -> Result<()> {
    let conn = storage::open_connection();
    create_database_dir().and_then(|_| init_database(&conn))
}

fn create_database_dir() -> Result<()> {
    super::database_file_path()
        .parent()
        .map(|database_file_dir| {
            if !database_file_dir.exists() {
                DirBuilder::new().create(database_file_dir).unwrap();
            }
            ()
        })
        .ok_or_else(|| Error::msg("Failed to create database dir."))
}

fn init_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo  (
            id              INTEGER PRIMARY KEY,
            title           TEXT NOT NULL,
            done            INTEGER DEFAULT 0,
            today           TEXT DEFAULT '',
            list            TEXT DEFAULT '',
            priority        TEXT DEFAULT 'medium',
            created_at      INTEGER DEFAULT 0,
            completed_at    INTEGER DEFAULT 0
            )",
        params![],
    )?;
    Ok(())
}
