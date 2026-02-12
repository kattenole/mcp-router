use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rusqlite::params;
use uuid::Uuid;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::{LoginResponse, User, ValidateSessionResponse};

/// Session expiry duration: 7 days in seconds.
const SESSION_EXPIRY_SECS: i64 = 7 * 24 * 60 * 60;

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Auth(format!("Failed to hash password: {}", e)))?;
        Ok(hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Auth(format!("Invalid hash: {}", e)))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn setup_default_user(db: &Database) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
        if count == 0 {
            let hash = Self::hash_password("admin")?;
            let id = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO users (id, username, password_hash) VALUES (?1, ?2, ?3)",
                params![id, "admin", hash],
            )?;
            log::info!("Default admin user created (username: admin, password: admin)");
        }
        Ok(())
    }

    pub fn login(db: &Database, username: &str, password: &str) -> Result<LoginResponse, AppError> {
        let conn = db.conn.lock().unwrap();
        let user: User = conn
            .query_row(
                "SELECT id, username, password_hash, created_at, updated_at FROM users WHERE username = ?1",
                params![username],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password_hash: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                },
            )
            .map_err(|_| AppError::Auth("Invalid username or password".to_string()))?;

        if !Self::verify_password(password, &user.password_hash)? {
            return Err(AppError::Auth("Invalid username or password".to_string()));
        }

        // Generate a cryptographically secure session token
        use rand::RngCore;
        let mut token_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut token_bytes);
        let token = format!("mcpr_{}", base64_url_encode(&token_bytes));

        // Store a SHA-256 hash of the token in the sessions table
        let token_hash = sha256_hex(&token);
        let expires_at = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(SESSION_EXPIRY_SECS))
            .unwrap_or_else(chrono::Utc::now)
            .to_rfc3339();

        // Remove any existing sessions for this user (single-session policy)
        conn.execute(
            "DELETE FROM sessions WHERE user_id = ?1",
            params![user.id],
        )?;

        conn.execute(
            "INSERT INTO sessions (token_hash, user_id, username, expires_at) VALUES (?1, ?2, ?3, ?4)",
            params![token_hash, user.id, user.username, expires_at],
        )?;

        Ok(LoginResponse {
            token,
            username: user.username,
        })
    }

    /// Validate a session token against the database.
    /// Returns the associated username if the session is valid and not expired.
    pub fn validate_session(db: &Database, token: &str) -> Result<ValidateSessionResponse, AppError> {
        let conn = db.conn.lock().unwrap();
        let token_hash = sha256_hex(token);

        let result = conn.query_row(
            "SELECT username, expires_at FROM sessions WHERE token_hash = ?1",
            params![token_hash],
            |row| {
                let username: String = row.get(0)?;
                let expires_at: String = row.get(1)?;
                Ok((username, expires_at))
            },
        );

        match result {
            Ok((username, expires_at)) => {
                // Check if the session has expired
                if let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(&expires_at) {
                    if expiry < chrono::Utc::now() {
                        // Clean up expired session
                        let _ = conn.execute(
                            "DELETE FROM sessions WHERE token_hash = ?1",
                            params![token_hash],
                        );
                        return Ok(ValidateSessionResponse {
                            valid: false,
                            username: None,
                        });
                    }
                }
                Ok(ValidateSessionResponse {
                    valid: true,
                    username: Some(username),
                })
            }
            Err(_) => Ok(ValidateSessionResponse {
                valid: false,
                username: None,
            }),
        }
    }

    /// Invalidate a session token (logout).
    pub fn logout(db: &Database, token: &str) -> Result<(), AppError> {
        let conn = db.conn.lock().unwrap();
        let token_hash = sha256_hex(token);
        conn.execute(
            "DELETE FROM sessions WHERE token_hash = ?1",
            params![token_hash],
        )?;
        Ok(())
    }

    pub fn change_password(
        db: &Database,
        username: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        // First verify old password
        let conn = db.conn.lock().unwrap();
        let hash: String = conn
            .query_row(
                "SELECT password_hash FROM users WHERE username = ?1",
                params![username],
                |row| row.get(0),
            )
            .map_err(|_| AppError::Auth("User not found".to_string()))?;

        if !Self::verify_password(old_password, &hash)? {
            return Err(AppError::Auth("Invalid old password".to_string()));
        }

        let new_hash = Self::hash_password(new_password)?;
        conn.execute(
            "UPDATE users SET password_hash = ?1, updated_at = datetime('now') WHERE username = ?2",
            params![new_hash, username],
        )?;

        // Invalidate all existing sessions for this user after password change
        conn.execute(
            "DELETE FROM sessions WHERE username = ?1",
            params![username],
        )?;

        Ok(())
    }

    pub fn has_users(db: &Database) -> Result<bool, AppError> {
        let conn = db.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
        Ok(count > 0)
    }
}

fn base64_url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

fn sha256_hex(input: &str) -> String {
    use std::fmt::Write;
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in result.iter() {
        let _ = write!(hex, "{:02x}", byte);
    }
    hex
}
