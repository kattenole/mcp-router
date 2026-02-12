use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::*;

pub struct LoggingService;

impl LoggingService {
    pub fn log_request(
        db: &Database,
        server_id: Option<&str>,
        server_name: Option<&str>,
        client_id: Option<&str>,
        client_name: Option<&str>,
        request_type: &str,
        request_params: Option<&str>,
        response_status: &str,
        duration: Option<i64>,
        error_message: Option<&str>,
    ) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp_millis();

        conn.execute(
            "INSERT INTO request_logs (id, timestamp, client_id, client_name, server_id, server_name, request_type, request_params, response_status, duration, error_message)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![id, timestamp, client_id, client_name, server_id, server_name, request_type, request_params, response_status, duration, error_message],
        )?;

        Ok(())
    }

    pub fn query_logs(db: &Database, params_q: LogQueryParams) -> Result<LogQueryResult, AppError> {
        let conn = db.conn.lock().unwrap();
        let limit = params_q.limit.unwrap_or(50);
        let offset = params_q.offset.unwrap_or(0);

        let mut where_clauses = Vec::new();
        let mut query_params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        let mut param_idx = 1;

        if let Some(ref server_id) = params_q.server_id {
            where_clauses.push(format!("server_id = ?{}", param_idx));
            query_params.push(Box::new(server_id.clone()));
            param_idx += 1;
        }
        if let Some(ref request_type) = params_q.request_type {
            where_clauses.push(format!("request_type = ?{}", param_idx));
            query_params.push(Box::new(request_type.clone()));
            param_idx += 1;
        }
        if let Some(ref response_status) = params_q.response_status {
            where_clauses.push(format!("response_status = ?{}", param_idx));
            query_params.push(Box::new(response_status.clone()));
            let _ = param_idx;
        }

        let where_sql = if where_clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        };

        // Get total count
        let count_sql = format!("SELECT COUNT(*) FROM request_logs {}", where_sql);
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = query_params.iter().map(|p| p.as_ref()).collect();
        let total: i64 = conn.query_row(&count_sql, param_refs.as_slice(), |row| row.get(0))?;

        // Get items
        let items_sql = format!(
            "SELECT id, timestamp, client_id, client_name, server_id, server_name, request_type, request_params, response_status, duration, error_message
             FROM request_logs {} ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            where_sql, limit, offset
        );

        let param_refs2: Vec<&dyn rusqlite::types::ToSql> = query_params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&items_sql)?;
        let items = stmt.query_map(param_refs2.as_slice(), |row| {
            Ok(RequestLog {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                client_id: row.get(2)?,
                client_name: row.get(3)?,
                server_id: row.get(4)?,
                server_name: row.get(5)?,
                request_type: row.get(6)?,
                request_params: row.get(7)?,
                response_status: row.get(8)?,
                duration: row.get(9)?,
                error_message: row.get(10)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(LogQueryResult {
            has_more: (offset + limit) < total,
            items,
            total,
        })
    }

    pub fn clear_logs(db: &Database) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        conn.execute("DELETE FROM request_logs", [])?;
        Ok(())
    }

    pub fn get_stats(db: &Database) -> Result<AppStats, AppError> {
        let conn = db.conn.lock().unwrap();

        let total_servers: i64 = conn.query_row("SELECT COUNT(*) FROM servers", [], |row| row.get(0))?;
        let active_servers: i64 = conn.query_row("SELECT COUNT(*) FROM servers WHERE disabled = 0", [], |row| row.get(0))?;
        let total_requests: i64 = conn.query_row("SELECT COUNT(*) FROM request_logs", [], |row| row.get(0))?;
        let total_projects: i64 = conn.query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))?;

        Ok(AppStats {
            total_servers,
            active_servers,
            total_requests,
            total_projects,
        })
    }
}
