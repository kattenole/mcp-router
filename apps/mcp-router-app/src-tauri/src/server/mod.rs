pub mod projects;

use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::*;

pub struct ServerService;

impl ServerService {
    pub fn create_server(db: &Database, req: CreateServerRequest) -> Result<McpServer, AppError> {
        let conn = db.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let server_type_str = req.server_type.to_string();
        let auto_start = req.auto_start.unwrap_or(false);

        conn.execute(
            "INSERT INTO servers (id, name, description, server_type, command, args, env, remote_url, bearer_token, project_id, auto_start, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                id,
                req.name,
                req.description,
                server_type_str,
                req.command,
                req.args,
                req.env,
                req.remote_url,
                req.bearer_token,
                req.project_id,
                auto_start as i32,
                now,
                now,
            ],
        )?;

        let server = McpServer {
            id,
            name: req.name,
            description: req.description,
            server_type: req.server_type,
            command: req.command,
            args: req.args,
            env: req.env,
            remote_url: req.remote_url,
            bearer_token: req.bearer_token,
            project_id: req.project_id,
            tool_permissions: None,
            auto_start,
            disabled: false,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(server)
    }

    pub fn list_servers(db: &Database) -> Result<Vec<McpServer>, AppError> {
        let conn = db.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, server_type, command, args, env, remote_url, bearer_token, project_id, tool_permissions, auto_start, disabled, created_at, updated_at
             FROM servers ORDER BY created_at DESC"
        )?;

        let servers = stmt.query_map([], |row| {
            let server_type_str: String = row.get(3)?;
            Ok(McpServer {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                server_type: server_type_str.parse().unwrap_or(ServerType::Local),
                command: row.get(4)?,
                args: row.get(5)?,
                env: row.get(6)?,
                remote_url: row.get(7)?,
                bearer_token: row.get(8)?,
                project_id: row.get(9)?,
                tool_permissions: row.get(10)?,
                auto_start: row.get::<_, i32>(11)? != 0,
                disabled: row.get::<_, i32>(12)? != 0,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(servers)
    }

    pub fn get_server(db: &Database, server_id: &str) -> Result<McpServer, AppError> {
        let conn = db.conn.lock().unwrap();
        let server = conn.query_row(
            "SELECT id, name, description, server_type, command, args, env, remote_url, bearer_token, project_id, tool_permissions, auto_start, disabled, created_at, updated_at
             FROM servers WHERE id = ?1",
            params![server_id],
            |row| {
                let server_type_str: String = row.get(3)?;
                Ok(McpServer {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    server_type: server_type_str.parse().unwrap_or(ServerType::Local),
                    command: row.get(4)?,
                    args: row.get(5)?,
                    env: row.get(6)?,
                    remote_url: row.get(7)?,
                    bearer_token: row.get(8)?,
                    project_id: row.get(9)?,
                    tool_permissions: row.get(10)?,
                    auto_start: row.get::<_, i32>(11)? != 0,
                    disabled: row.get::<_, i32>(12)? != 0,
                    created_at: row.get(13)?,
                    updated_at: row.get(14)?,
                })
            },
        ).map_err(|_| AppError::NotFound(format!("Server {} not found", server_id)))?;

        Ok(server)
    }

    pub fn update_server(db: &Database, req: UpdateServerRequest) -> Result<McpServer, AppError> {
        let conn = db.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        // Build dynamic update
        let mut updates = vec!["updated_at = ?1".to_string()];
        let mut param_idx = 2;
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(now.clone())];

        macro_rules! add_update {
            ($field:expr, $col:expr) => {
                if let Some(ref val) = $field {
                    updates.push(format!("{} = ?{}", $col, param_idx));
                    param_values.push(Box::new(val.clone()));
                    param_idx += 1;
                }
            };
        }

        add_update!(req.name, "name");
        add_update!(req.description, "description");
        add_update!(req.command, "command");
        add_update!(req.args, "args");
        add_update!(req.env, "env");
        add_update!(req.remote_url, "remote_url");
        add_update!(req.bearer_token, "bearer_token");
        add_update!(req.project_id, "project_id");
        add_update!(req.tool_permissions, "tool_permissions");

        if let Some(auto_start) = req.auto_start {
            updates.push(format!("auto_start = ?{}", param_idx));
            param_values.push(Box::new(auto_start as i32));
            param_idx += 1;
        }
        if let Some(disabled) = req.disabled {
            updates.push(format!("disabled = ?{}", param_idx));
            param_values.push(Box::new(disabled as i32));
            let _ = param_idx;
        }

        let sql = format!(
            "UPDATE servers SET {} WHERE id = ?{}",
            updates.join(", "),
            param_values.len() + 1
        );
        param_values.push(Box::new(req.id.clone()));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        conn.execute(&sql, param_refs.as_slice())?;
        drop(conn);

        Self::get_server(db, &req.id)
    }

    pub fn delete_server(db: &Database, server_id: &str) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM servers WHERE id = ?1", params![server_id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("Server {} not found", server_id)));
        }
        Ok(())
    }

    pub fn toggle_server(db: &Database, server_id: &str, disabled: bool) -> Result<McpServer, AppError> {
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "UPDATE servers SET disabled = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![disabled as i32, server_id],
        )?;
        drop(conn);
        Self::get_server(db, server_id)
    }

    pub fn update_tool_permissions(db: &Database, server_id: &str, permissions: &str) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        let affected = conn.execute(
            "UPDATE servers SET tool_permissions = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![permissions, server_id],
        )?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("Server {} not found", server_id)));
        }
        Ok(())
    }
}
