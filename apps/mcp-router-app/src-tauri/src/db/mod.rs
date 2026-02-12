use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::sync::Mutex;
use crate::errors::AppError;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self, AppError> {
        let conn = Connection::open(&db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS migrations (
                id TEXT PRIMARY KEY,
                executed_at TEXT NOT NULL DEFAULT (datetime('now'))
            );"
        )?;

        let migrations = vec![
            ("001_initial", include_str!("migrations/001_initial.sql")),
        ];

        for (id, sql) in migrations {
            let exists: bool = conn.query_row(
                "SELECT COUNT(*) > 0 FROM migrations WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )?;
            if !exists {
                conn.execute_batch(sql)?;
                conn.execute(
                    "INSERT INTO migrations (id) VALUES (?1)",
                    params![id],
                )?;
                log::info!("Ran migration: {}", id);
            }
        }
        Ok(())
    }
}
