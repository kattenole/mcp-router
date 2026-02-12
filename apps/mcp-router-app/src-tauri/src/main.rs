#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

use mcp_router_lib::errors::AppError;
use mcp_router_lib::models::*;
use mcp_router_lib::mcp::McpClient;
use mcp_router_lib::{auth, db, logging, server, token, AppState};

// ============== Auth Commands ==============

#[tauri::command]
async fn login(
    state: tauri::State<'_, AppState>,
    username: String,
    password: String,
) -> Result<LoginResponse, AppError> {
    auth::AuthService::login(&state.db, &username, &password)
}

#[tauri::command]
async fn change_password(
    state: tauri::State<'_, AppState>,
    username: String,
    old_password: String,
    new_password: String,
) -> Result<(), AppError> {
    auth::AuthService::change_password(&state.db, &username, &old_password, &new_password)
}

#[tauri::command]
async fn has_users(state: tauri::State<'_, AppState>) -> Result<bool, AppError> {
    auth::AuthService::has_users(&state.db)
}

// ============== Server Commands ==============

#[tauri::command]
async fn create_server(
    state: tauri::State<'_, AppState>,
    req: CreateServerRequest,
) -> Result<McpServer, AppError> {
    server::ServerService::create_server(&state.db, req)
}

#[tauri::command]
async fn list_servers(state: tauri::State<'_, AppState>) -> Result<Vec<McpServer>, AppError> {
    server::ServerService::list_servers(&state.db)
}

#[tauri::command]
async fn get_server(
    state: tauri::State<'_, AppState>,
    server_id: String,
) -> Result<McpServer, AppError> {
    server::ServerService::get_server(&state.db, &server_id)
}

#[tauri::command]
async fn update_server(
    state: tauri::State<'_, AppState>,
    req: UpdateServerRequest,
) -> Result<McpServer, AppError> {
    server::ServerService::update_server(&state.db, req)
}

#[tauri::command]
async fn delete_server(
    state: tauri::State<'_, AppState>,
    server_id: String,
) -> Result<(), AppError> {
    let client_arc = {
        let mut clients = state.clients.lock().await;
        clients.remove(&server_id)
    };
    if let Some(client_arc) = client_arc {
        let _ = client_arc.lock().await.close().await;
    }
    server::ServerService::delete_server(&state.db, &server_id)
}

#[tauri::command]
async fn toggle_server(
    state: tauri::State<'_, AppState>,
    server_id: String,
    disabled: bool,
) -> Result<McpServer, AppError> {
    if disabled {
        let client_arc = {
            let mut clients = state.clients.lock().await;
            clients.remove(&server_id)
        };
        if let Some(client_arc) = client_arc {
            let _ = client_arc.lock().await.close().await;
        }
    }
    server::ServerService::toggle_server(&state.db, &server_id, disabled)
}

#[tauri::command]
async fn update_tool_permissions(
    state: tauri::State<'_, AppState>,
    server_id: String,
    permissions: String,
) -> Result<(), AppError> {
    server::ServerService::update_tool_permissions(&state.db, &server_id, &permissions)
}

// ============== MCP Connection Commands ==============

#[tauri::command]
async fn start_server(
    state: tauri::State<'_, AppState>,
    server_id: String,
) -> Result<ServerStatus, AppError> {
    let srv = server::ServerService::get_server(&state.db, &server_id)?;

    let client = match srv.server_type {
        ServerType::Local => {
            let command = srv.command.as_deref().ok_or_else(|| {
                AppError::Mcp("No command specified for local server".to_string())
            })?;
            let args: Vec<String> = srv
                .args
                .as_deref()
                .map(|a| serde_json::from_str(a).unwrap_or_default())
                .unwrap_or_default();
            let env: HashMap<String, String> = srv
                .env
                .as_deref()
                .map(|e| serde_json::from_str(e).unwrap_or_default())
                .unwrap_or_default();
            McpClient::connect_stdio(&srv.id, &srv.name, command, &args, env).await?
        }
        ServerType::Remote | ServerType::RemoteStreamable => {
            let url = srv.remote_url.as_deref().ok_or_else(|| {
                AppError::Mcp("No URL specified for remote server".to_string())
            })?;
            McpClient::connect_http(&srv.id, &srv.name, url, srv.bearer_token.clone()).await?
        }
    };

    let status = ServerStatus {
        server_id: srv.id.clone(),
        is_running: true,
        tools: client.tools.clone(),
        error: None,
    };

    let _ = logging::LoggingService::log_request(
        &state.db, Some(&srv.id), Some(&srv.name),
        None, None, "StartServer", None, "success", None, None,
    );

    state.clients.lock().await.insert(srv.id.clone(), Arc::new(Mutex::new(client)));
    Ok(status)
}

#[tauri::command]
async fn stop_server(
    state: tauri::State<'_, AppState>,
    server_id: String,
) -> Result<(), AppError> {
    let client_arc = {
        let mut clients = state.clients.lock().await;
        clients.remove(&server_id)
    };
    if let Some(client_arc) = client_arc {
        let mut client = client_arc.lock().await;
        client.close().await?;
        let _ = logging::LoggingService::log_request(
            &state.db, Some(&server_id), Some(&client.server_name),
            None, None, "StopServer", None, "success", None, None,
        );
    }
    Ok(())
}

#[tauri::command]
async fn get_server_status(
    state: tauri::State<'_, AppState>,
    server_id: String,
) -> Result<ServerStatus, AppError> {
    let client_arc = {
        let clients = state.clients.lock().await;
        clients.get(&server_id).cloned()
    };
    Ok(if let Some(client_arc) = client_arc {
        let client = client_arc.lock().await;
        ServerStatus {
            server_id: server_id.clone(),
            is_running: true,
            tools: client.tools.clone(),
            error: None,
        }
    } else {
        ServerStatus { server_id, is_running: false, tools: vec![], error: None }
    })
}

#[tauri::command]
async fn list_server_tools(
    state: tauri::State<'_, AppState>,
    server_id: String,
) -> Result<Vec<McpTool>, AppError> {
    let client_arc = {
        let clients = state.clients.lock().await;
        clients.get(&server_id).cloned()
    };
    if let Some(client_arc) = client_arc {
        client_arc.lock().await.list_tools().await
    } else {
        Err(AppError::Server(format!("Server {} is not running", server_id)))
    }
}

#[tauri::command]
async fn call_tool(
    state: tauri::State<'_, AppState>,
    server_id: String,
    tool_name: String,
    arguments: Option<serde_json::Value>,
) -> Result<serde_json::Value, AppError> {
    let start = std::time::Instant::now();
    let client_arc = {
        let clients = state.clients.lock().await;
        clients.get(&server_id).cloned().ok_or_else(|| {
            AppError::Server(format!("Server {} is not running", server_id))
        })?
    };

    let mut client = client_arc.lock().await;
    let result = client.call_tool(&tool_name, arguments.clone()).await;
    let duration = start.elapsed().as_millis() as i64;
    let params_json = serde_json::json!({"tool": tool_name, "arguments": arguments}).to_string();

    match &result {
        Ok(r) => {
            let _ = logging::LoggingService::log_request(
                &state.db, Some(&server_id), Some(&client.server_name),
                None, None, "CallTool", Some(&params_json), "success", Some(duration), None,
            );
            Ok(serde_json::to_value(r).unwrap_or_default())
        }
        Err(e) => {
            let _ = logging::LoggingService::log_request(
                &state.db, Some(&server_id), Some(&client.server_name),
                None, None, "CallTool", Some(&params_json), "error", Some(duration), Some(&e.to_string()),
            );
            Err(AppError::Mcp(e.to_string()))
        }
    }
}

#[tauri::command]
async fn get_all_server_statuses(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ServerStatus>, AppError> {
    let servers = server::ServerService::list_servers(&state.db)?;
    let client_arcs: Vec<_> = {
        let clients = state.clients.lock().await;
        servers.iter().map(|s| {
            (s.id.clone(), clients.get(&s.id).cloned())
        }).collect()
    };
    let mut statuses = Vec::with_capacity(client_arcs.len());
    for (server_id, client_arc) in client_arcs {
        if let Some(client_arc) = client_arc {
            let client = client_arc.lock().await;
            statuses.push(ServerStatus {
                server_id,
                is_running: true,
                tools: client.tools.clone(),
                error: None,
            });
        } else {
            statuses.push(ServerStatus {
                server_id,
                is_running: false,
                tools: vec![],
                error: None,
            });
        }
    }
    Ok(statuses)
}

// ============== Project Commands ==============

#[tauri::command]
async fn create_project(
    state: tauri::State<'_, AppState>,
    req: CreateProjectRequest,
) -> Result<Project, AppError> {
    server::projects::ProjectService::create_project(&state.db, req)
}

#[tauri::command]
async fn list_projects(state: tauri::State<'_, AppState>) -> Result<Vec<Project>, AppError> {
    server::projects::ProjectService::list_projects(&state.db)
}

#[tauri::command]
async fn get_project(
    state: tauri::State<'_, AppState>,
    project_id: String,
) -> Result<Project, AppError> {
    server::projects::ProjectService::get_project(&state.db, &project_id)
}

#[tauri::command]
async fn delete_project(
    state: tauri::State<'_, AppState>,
    project_id: String,
) -> Result<(), AppError> {
    server::projects::ProjectService::delete_project(&state.db, &project_id)
}

#[tauri::command]
async fn update_project(
    state: tauri::State<'_, AppState>,
    project_id: String,
    name: Option<String>,
    optimization: Option<String>,
) -> Result<Project, AppError> {
    server::projects::ProjectService::update_project(&state.db, &project_id, name, optimization)
}

// ============== Token Commands ==============

#[tauri::command]
async fn generate_token(
    state: tauri::State<'_, AppState>,
    req: GenerateTokenRequest,
) -> Result<ApiToken, AppError> {
    token::TokenService::generate_token(&state.db, req)
}

#[tauri::command]
async fn list_tokens(state: tauri::State<'_, AppState>) -> Result<Vec<ApiToken>, AppError> {
    token::TokenService::list_tokens(&state.db)
}

#[tauri::command]
async fn revoke_token(
    state: tauri::State<'_, AppState>,
    token_id: String,
) -> Result<(), AppError> {
    token::TokenService::revoke_token(&state.db, &token_id)
}

// ============== Logging Commands ==============

#[tauri::command]
async fn query_logs(
    state: tauri::State<'_, AppState>,
    params: LogQueryParams,
) -> Result<LogQueryResult, AppError> {
    logging::LoggingService::query_logs(&state.db, params)
}

#[tauri::command]
async fn clear_logs(state: tauri::State<'_, AppState>) -> Result<(), AppError> {
    logging::LoggingService::clear_logs(&state.db)
}

#[tauri::command]
async fn get_stats(state: tauri::State<'_, AppState>) -> Result<AppStats, AppError> {
    logging::LoggingService::get_stats(&state.db)
}

// ============== Main ==============

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");

            let db_path = app_dir.join("mcp-router.db");
            log::info!("Database path: {:?}", db_path);

            let database = db::Database::new(db_path).expect("Failed to initialize database");
            auth::AuthService::setup_default_user(&database).expect("Failed to setup default user");

            app.manage(AppState {
                db: database,
                clients: Arc::new(Mutex::new(HashMap::new())),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            login, change_password, has_users,
            create_server, list_servers, get_server, update_server, delete_server,
            toggle_server, update_tool_permissions,
            start_server, stop_server, get_server_status, list_server_tools,
            call_tool, get_all_server_statuses,
            create_project, list_projects, get_project, delete_project, update_project,
            generate_token, list_tokens, revoke_token,
            query_logs, clear_logs, get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
