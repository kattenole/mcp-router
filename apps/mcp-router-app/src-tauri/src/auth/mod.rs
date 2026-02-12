use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rusqlite::params;
use uuid::Uuid;

use crate::db::Database;
use crate::errors::AppError;
use crate::models::{LoginResponse, User};

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

        // Generate a session token
        let token = format!("mcpr_{}", Uuid::new_v4().to_string().replace("-", ""));

        Ok(LoginResponse {
            token,
            username: user.username,
        })
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

        Ok(())
    }

    pub fn has_users(db: &Database) -> Result<bool, AppError> {
        let conn = db.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
        Ok(count > 0)
    }
}
