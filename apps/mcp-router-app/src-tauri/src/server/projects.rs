use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::*;

pub struct ProjectService;

impl ProjectService {
    pub fn create_project(db: &Database, req: CreateProjectRequest) -> Result<Project, AppError> {
        let conn = db.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO projects (id, name, optimization, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, req.name, req.optimization, now, now],
        )?;

        Ok(Project {
            id,
            name: req.name,
            optimization: req.optimization,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn list_projects(db: &Database) -> Result<Vec<Project>, AppError> {
        let conn = db.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, optimization, created_at, updated_at FROM projects ORDER BY created_at DESC"
        )?;

        let projects = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                optimization: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub fn get_project(db: &Database, project_id: &str) -> Result<Project, AppError> {
        let conn = db.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, optimization, created_at, updated_at FROM projects WHERE id = ?1",
            params![project_id],
            |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    optimization: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        ).map_err(|_| AppError::NotFound(format!("Project {} not found", project_id)))
    }

    pub fn delete_project(db: &Database, project_id: &str) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        let tx = conn.unchecked_transaction()?;
        // Unassign servers from this project
        tx.execute(
            "UPDATE servers SET project_id = NULL WHERE project_id = ?1",
            params![project_id],
        )?;
        let affected = tx.execute("DELETE FROM projects WHERE id = ?1", params![project_id])?;
        if affected == 0 {
            tx.rollback().ok();
            return Err(AppError::NotFound(format!("Project {} not found", project_id)));
        }
        tx.commit()?;
        Ok(())
    }

    pub fn update_project(db: &Database, project_id: &str, name: Option<String>, optimization: Option<String>) -> Result<Project, AppError> {
        let conn = db.conn.lock().unwrap();
        if let Some(ref n) = name {
            conn.execute(
                "UPDATE projects SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
                params![n, project_id],
            )?;
        }
        if let Some(ref o) = optimization {
            conn.execute(
                "UPDATE projects SET optimization = ?1, updated_at = datetime('now') WHERE id = ?2",
                params![o, project_id],
            )?;
        }
        drop(conn);
        Self::get_project(db, project_id)
    }
}
