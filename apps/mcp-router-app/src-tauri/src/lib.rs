pub mod auth;
pub mod db;
pub mod errors;
pub mod logging;
pub mod mcp;
pub mod models;
pub mod server;
pub mod token;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use db::Database;
use mcp::McpClient;

pub struct AppState {
    pub db: Database,
    pub clients: Arc<Mutex<HashMap<String, McpClient>>>,
}
