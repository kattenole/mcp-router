use rusqlite::params;
use chrono::Utc;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::*;

pub struct TokenService;

impl TokenService {
    pub fn generate_token(db: &Database, req: GenerateTokenRequest) -> Result<ApiToken, AppError> {
        let conn = db.conn.lock().unwrap();
        // Use 24 bytes (192 bits) of cryptographic randomness
        use rand::RngCore;
        let mut token_bytes = [0u8; 24];
        rand::thread_rng().fill_bytes(&mut token_bytes);
        let id = format!("mcpr_{}", base64_url_encode(&token_bytes));
        let now = Utc::now().timestamp();
        let server_access = serde_json::to_string(&req.server_access)
            .map_err(|e| AppError::Serde(e))?;

        // Remove existing token for same client
        conn.execute(
            "DELETE FROM api_tokens WHERE client_id = ?1",
            params![req.client_id],
        )?;

        conn.execute(
            "INSERT INTO api_tokens (id, client_id, server_access, issued_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, req.client_id, server_access, now],
        )?;

        Ok(ApiToken {
            id,
            client_id: req.client_id,
            server_access,
            issued_at: now,
        })
    }

    pub fn list_tokens(db: &Database) -> Result<Vec<ApiToken>, AppError> {
        let conn = db.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, client_id, server_access, issued_at FROM api_tokens ORDER BY issued_at DESC"
        )?;

        let tokens = stmt.query_map([], |row| {
            Ok(ApiToken {
                id: row.get(0)?,
                client_id: row.get(1)?,
                server_access: row.get(2)?,
                issued_at: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(tokens)
    }

    pub fn revoke_token(db: &Database, token_id: &str) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM api_tokens WHERE id = ?1", params![token_id])?;
        if affected == 0 {
            return Err(AppError::NotFound("Token not found".to_string()));
        }
        Ok(())
    }

    pub fn validate_token(db: &Database, token_id: &str) -> Result<ApiToken, AppError> {
        let conn = db.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, client_id, server_access, issued_at FROM api_tokens WHERE id = ?1",
            params![token_id],
            |row| {
                Ok(ApiToken {
                    id: row.get(0)?,
                    client_id: row.get(1)?,
                    server_access: row.get(2)?,
                    issued_at: row.get(3)?,
                })
            },
        ).map_err(|_| AppError::Unauthorized)
    }
}

fn base64_url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}
