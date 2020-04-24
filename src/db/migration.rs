use anyhow::Error as AnyError;
use anyhow::Result as AnyResult;
use migrant_lib::Direction::Up;
use migrant_lib::{Config, EmbeddedMigration, Migrator, Settings};
use std::error::Error;
use std::fs::DirBuilder;

pub fn migrate() -> AnyResult<()> {
    create_database_dir()
        .and_then(|_| apply_migrations().map_err(|error| AnyError::msg(error.to_string())))
}

fn create_database_dir() -> AnyResult<()> {
    super::database_file_path()
        .parent()
        .map(|database_file_dir| {
            if !database_file_dir.exists() {
                DirBuilder::new().create(database_file_dir).unwrap();
            }
        })
        .ok_or_else(|| AnyError::msg("Failed to create database dir."))
}

fn apply_migrations() -> Result<(), Box<dyn Error>> {
    let settings = Settings::configure_sqlite()
        .database_path(super::database_file_path())?
        .build()?;

    let mut config = Config::with_settings(&settings);
    config.setup()?;

    config.use_migrations(&[
        EmbeddedMigration::with_tag("add-todo-table")
            .up(String::from(
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
            ))
            .boxed(),
        EmbeddedMigration::with_tag("add-due-date")
            .up(String::from(
                "ALTER TABLE todo ADD COLUMN due_date TEXT DEFAULT '';",
            ))
            .boxed(),
    ])?;

    let config = config.reload()?;

    Migrator::with_config(&config)
        .all(true)
        .direction(Up)
        .show_output(false)
        .swallow_completion(true)
        .apply()?;
    Ok(())
}
