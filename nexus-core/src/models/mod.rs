use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Credential {
    pub id: Uuid,
    pub name: String,
    pub provider: String,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExecutionRecord {
    pub id: Uuid,
    pub workflow_id: String,
    pub workflow_name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub results: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeExecutionResult {
    pub node_id: String,
    pub node_name: String,
    pub success: bool,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub kind: String,
    pub label: String,
    pub position: Position,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub from: Uuid,
    pub to: Uuid,
    #[serde(default, rename = "fromPort")]
    pub from_port: Option<String>,
    #[serde(default, rename = "toPort")]
    pub to_port: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct McpServer {
    pub id: Uuid,
    pub name: String,
    pub transport: String,
    pub command: Option<String>,
    pub args: serde_json::Value,
    pub endpoint: Option<String>,
    pub env: serde_json::Value,
    pub headers: serde_json::Value,
    pub auto_start: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

// Input DTOs
#[derive(Debug, Deserialize)]
pub struct WorkflowInput {
    pub name: String,
    pub description: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Deserialize)]
pub struct McpServerInput {
    pub name: String,
    pub transport: String,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub endpoint: Option<String>,
    pub env: Vec<EnvVar>,
    pub headers: Option<serde_json::Value>,
    pub auto_start: bool,
}

#[derive(Debug, Deserialize)]
pub struct CredentialInput {
    pub name: String,
    pub provider: String,
    pub data: serde_json::Value,
}
