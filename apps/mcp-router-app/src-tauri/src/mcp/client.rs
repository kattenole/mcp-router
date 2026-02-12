use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use serde_json::Value;

use super::protocol::*;
use crate::errors::AppError;
use crate::models::McpTool;

/// Represents an active MCP client connection
pub struct McpClient {
    pub server_id: String,
    pub server_name: String,
    transport: McpTransport,
    pub server_info: Option<InitializeResult>,
    pub tools: Vec<McpTool>,
    request_id: Arc<Mutex<i64>>,
}

enum McpTransport {
    Stdio {
        child: Child,
        stdin: Arc<Mutex<tokio::process::ChildStdin>>,
        stdout: Arc<Mutex<BufReader<tokio::process::ChildStdout>>>,
    },
    Http {
        url: String,
        bearer_token: Option<String>,
        client: reqwest::Client,
    },
}

impl McpClient {
    /// Connect to a local MCP server via stdio
    pub async fn connect_stdio(
        server_id: &str,
        server_name: &str,
        command: &str,
        args: &[String],
        env: HashMap<String, String>,
    ) -> Result<Self, AppError> {
        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .envs(env);

        let mut child = cmd.spawn().map_err(|e| {
            AppError::Mcp(format!("Failed to spawn process '{}': {}", command, e))
        })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            AppError::Mcp("Failed to capture stdin".to_string())
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            AppError::Mcp("Failed to capture stdout".to_string())
        })?;

        let mut client = Self {
            server_id: server_id.to_string(),
            server_name: server_name.to_string(),
            transport: McpTransport::Stdio {
                child,
                stdin: Arc::new(Mutex::new(stdin)),
                stdout: Arc::new(Mutex::new(BufReader::new(stdout))),
            },
            server_info: None,
            tools: vec![],
            request_id: Arc::new(Mutex::new(1)),
        };

        client.initialize().await?;
        Ok(client)
    }

    /// Connect to a remote MCP server via HTTP (Streamable HTTP transport)
    pub async fn connect_http(
        server_id: &str,
        server_name: &str,
        url: &str,
        bearer_token: Option<String>,
    ) -> Result<Self, AppError> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let mut client = Self {
            server_id: server_id.to_string(),
            server_name: server_name.to_string(),
            transport: McpTransport::Http {
                url: url.to_string(),
                bearer_token,
                client: http_client,
            },
            server_info: None,
            tools: vec![],
            request_id: Arc::new(Mutex::new(1)),
        };

        client.initialize().await?;
        Ok(client)
    }

    async fn next_id(&self) -> i64 {
        let mut id = self.request_id.lock().await;
        let current = *id;
        *id += 1;
        current
    }

    async fn send_request(&mut self, request: &JsonRpcRequest) -> Result<JsonRpcResponse, AppError> {
        match &mut self.transport {
            McpTransport::Stdio { stdin, stdout, .. } => {
                let json = serde_json::to_string(request)?;
                let message = format!("{}\n", json);

                let mut stdin_lock = stdin.lock().await;
                stdin_lock.write_all(message.as_bytes()).await
                    .map_err(|e| AppError::Mcp(format!("Failed to write to stdin: {}", e)))?;
                stdin_lock.flush().await
                    .map_err(|e| AppError::Mcp(format!("Failed to flush stdin: {}", e)))?;
                drop(stdin_lock);

                // Read response line
                let mut stdout_lock = stdout.lock().await;
                let mut line = String::new();
                stdout_lock.read_line(&mut line).await
                    .map_err(|e| AppError::Mcp(format!("Failed to read from stdout: {}", e)))?;

                if line.is_empty() {
                    return Err(AppError::Mcp("Server closed connection".to_string()));
                }

                let response: JsonRpcResponse = serde_json::from_str(&line)
                    .map_err(|e| AppError::Mcp(format!("Invalid JSON-RPC response: {} (raw: {})", e, line.trim())))?;

                Ok(response)
            }
            McpTransport::Http { url, bearer_token, client } => {
                let mut req = client
                    .post(url.as_str())
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json, text/event-stream")
                    .header("MCP-Protocol-Version", MCP_PROTOCOL_VERSION);

                if let Some(ref token) = bearer_token {
                    req = req.header("Authorization", format!("Bearer {}", token));
                }

                let resp = req
                    .json(request)
                    .send()
                    .await?;

                if !resp.status().is_success() {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    return Err(AppError::Mcp(format!(
                        "HTTP {} from MCP server: {}",
                        status, body
                    )));
                }

                let body = resp.text().await?;
                let response: JsonRpcResponse = serde_json::from_str(&body)
                    .map_err(|e| AppError::Mcp(format!("Invalid JSON-RPC response: {}", e)))?;

                Ok(response)
            }
        }
    }

    async fn send_notification(&mut self, request: &JsonRpcRequest) -> Result<(), AppError> {
        match &mut self.transport {
            McpTransport::Stdio { stdin, .. } => {
                let json = serde_json::to_string(request)?;
                let message = format!("{}\n", json);

                let mut stdin_lock = stdin.lock().await;
                stdin_lock.write_all(message.as_bytes()).await
                    .map_err(|e| AppError::Mcp(format!("Failed to write notification: {}", e)))?;
                stdin_lock.flush().await
                    .map_err(|e| AppError::Mcp(format!("Failed to flush notification: {}", e)))?;
                Ok(())
            }
            McpTransport::Http { url, bearer_token, client } => {
                let mut req = client
                    .post(url.as_str())
                    .header("Content-Type", "application/json")
                    .header("MCP-Protocol-Version", MCP_PROTOCOL_VERSION);

                if let Some(ref token) = bearer_token {
                    req = req.header("Authorization", format!("Bearer {}", token));
                }

                let _resp = req.json(request).send().await?;
                Ok(())
            }
        }
    }

    async fn initialize(&mut self) -> Result<(), AppError> {
        let mut init_req = JsonRpcRequest::initialize();
        let id = self.next_id().await;
        init_req.id = Some(Value::Number(serde_json::Number::from(id)));

        let response = self.send_request(&init_req).await?;

        if let Some(error) = response.error {
            return Err(AppError::Mcp(format!(
                "Initialize failed: {} (code: {})",
                error.message, error.code
            )));
        }

        if let Some(result) = response.result {
            let init_result: InitializeResult = serde_json::from_value(result)
                .map_err(|e| AppError::Mcp(format!("Invalid initialize result: {}", e)))?;
            self.server_info = Some(init_result);
        }

        // Send initialized notification
        let notification = JsonRpcRequest::initialized_notification();
        self.send_notification(&notification).await?;

        Ok(())
    }

    pub async fn list_tools(&mut self) -> Result<Vec<McpTool>, AppError> {
        let mut req = JsonRpcRequest::list_tools();
        let id = self.next_id().await;
        req.id = Some(Value::Number(serde_json::Number::from(id)));

        let response = self.send_request(&req).await?;

        if let Some(error) = response.error {
            return Err(AppError::Mcp(format!(
                "List tools failed: {} (code: {})",
                error.message, error.code
            )));
        }

        let tools = if let Some(result) = response.result {
            let list_result: ListToolsResult = serde_json::from_value(result)
                .map_err(|e| AppError::Mcp(format!("Invalid list tools result: {}", e)))?;
            list_result
                .tools
                .into_iter()
                .map(|t| McpTool {
                    name: t.name,
                    description: t.description,
                    input_schema: t.input_schema,
                })
                .collect()
        } else {
            vec![]
        };

        self.tools = tools.clone();
        Ok(tools)
    }

    pub async fn call_tool(
        &mut self,
        tool_name: &str,
        arguments: Option<Value>,
    ) -> Result<CallToolResult, AppError> {
        let mut req = JsonRpcRequest::call_tool(tool_name, arguments);
        let id = self.next_id().await;
        req.id = Some(Value::Number(serde_json::Number::from(id)));

        let response = self.send_request(&req).await?;

        if let Some(error) = response.error {
            return Err(AppError::Mcp(format!(
                "Tool call failed: {} (code: {})",
                error.message, error.code
            )));
        }

        if let Some(result) = response.result {
            let call_result: CallToolResult = serde_json::from_value(result)
                .map_err(|e| AppError::Mcp(format!("Invalid call tool result: {}", e)))?;
            Ok(call_result)
        } else {
            Err(AppError::Mcp("No result from tool call".to_string()))
        }
    }

    pub async fn close(&mut self) -> Result<(), AppError> {
        match &mut self.transport {
            McpTransport::Stdio { child, .. } => {
                let _ = child.kill().await;
                Ok(())
            }
            McpTransport::Http { .. } => {
                // HTTP connections are stateless, nothing to close
                Ok(())
            }
        }
    }
}
