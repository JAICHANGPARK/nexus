use axum::{
    Json,
    extract::{Path, State, Form},
    http::StatusCode,
};

// ... (other imports)

pub async fn handle_slack_interactive(
    State(state): State<AppState>,
    Form(payload): Form<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    // Slack sends payload as a string in a form field
    let payload_str = payload.get("payload").and_then(|v| v.as_str()).ok_or(StatusCode::BAD_REQUEST)?;
    let payload_json: serde_json::Value = serde_json::from_str(payload_str).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let ts = payload_json.get("container").and_then(|v| v.get("message_ts")).and_then(|v| v.as_str());
    let channel = payload_json.get("channel").and_then(|v| v.get("id")).and_then(|v| v.as_str());
    let action_id = payload_json.get("actions").and_then(|v| v.get(0)).and_then(|v| v.get("action_id")).and_then(|v| v.as_str());

    if let (Some(ts), Some(_ch), Some(action)) = (ts, channel, action_id) {
        // Find the execution waiting for this message
        // In a real implementation, we'd search by TS and Channel in the snapshot data
        let execution = sqlx::query_as::<_, ExecutionRecord>(
            "SELECT * FROM executions WHERE status = 'waiting' AND snapshot->'wait_info'->>'ts' = $1 LIMIT 1"
        )
        .bind(ts)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Some(record) = execution {
            // Resume Execution
            let snapshot = record.snapshot.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut last_output = serde_json::json!({
                "action": action,
                "user": payload_json.get("user").and_then(|v| v.get("username")).and_then(|v| v.as_str()),
                "timestamp": chrono::Utc::now()
            });
            
            let remaining_nodes: Vec<Node> = serde_json::from_value(snapshot.get("remaining_nodes").cloned().unwrap_or_default()).unwrap_or_default();
            let mut results: Vec<NodeExecutionResult> = serde_json::from_value(record.results).unwrap_or_default();
            
            // Continue execution loop
            let mut success = true;
            for node in remaining_nodes {
                let node_start_time = std::time::Instant::now();
                match execute_single_node(&state.db, &node, &vec![], &vec![], &last_output).await {
                    Ok(output) => {
                        last_output = output.clone();
                        results.push(NodeExecutionResult {
                            node_id: node.id.to_string(),
                            node_name: node.label.clone(),
                            success: true,
                            output: Some(output),
                            error: None,
                            execution_time_ms: node_start_time.elapsed().as_millis() as u64,
                        });
                    }
                    Err(e) => {
                        success = false;
                        results.push(NodeExecutionResult {
                            node_id: node.id.to_string(),
                            node_name: node.label.clone(),
                            success: false,
                            output: None,
                            error: Some(e),
                            execution_time_ms: node_start_time.elapsed().as_millis() as u64,
                        });
                        break;
                    }
                }
            }

            // Update execution record to finished
            let _ = sqlx::query(
                "UPDATE executions SET status = $1, results = $2, end_time = NOW(), snapshot = NULL WHERE id = $3"
            )
            .bind(if success { "success" } else { "failed" })
            .bind(serde_json::to_value(&results).unwrap_or_default())
            .bind(record.id)
            .execute(&state.db)
            .await;

            return Ok(StatusCode::OK);
        }
    }

    Ok(StatusCode::OK)
}
use uuid::Uuid;
use crate::state::AppState;
use crate::models::*;
use crate::engine::execute_single_node;
use crate::clients::openai::OpenAiMessage;
use crate::clients::openrouter::{OpenRouterMessage, OpenRouterRequest};
use serde::{Deserialize, Serialize};

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct N8nWorkflowExport {
    pub name: String,
    pub nodes: Vec<N8nNode>,
    pub connections: serde_json::Value,
    pub settings: serde_json::Value,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct N8nNode {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(rename = "typeVersion")]
    pub type_version: i32,
    pub position: [f32; 2],
    pub parameters: serde_json::Value,
}

pub async fn export_workflow_n8n(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<N8nWorkflowExport>, StatusCode> {
    let workflow = sqlx::query_as::<_, Workflow>("SELECT * FROM workflows WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let nodes_list: Vec<Node> = serde_json::from_value(workflow.nodes).unwrap_or_default();
    let n8n_nodes: Vec<N8nNode> = nodes_list
        .iter()
        .map(|node| {
            let node_type = match node.kind.as_str() {
                "trigger-start" => "n8n-nodes-base.manualTrigger",
                "trigger-schedule" => "n8n-nodes-base.scheduleTrigger",
                "trigger-webhook" => "n8n-nodes-base.webhook",
                "http-request" => "n8n-nodes-base.httpRequest",
                "code" => "n8n-nodes-base.code",
                "set" => "n8n-nodes-base.set",
                "if" => "n8n-nodes-base.if",
                "switch" => "n8n-nodes-base.switch",
                "merge" => "n8n-nodes-base.merge",
                "filter" => "n8n-nodes-base.filter",
                "trigger-end" => "n8n-nodes-base.noOp",
                _ => "n8n-nodes-base.noOp",
            };

            N8nNode {
                id: node.id.to_string(),
                name: node.label.clone(),
                node_type: node_type.to_string(),
                type_version: 1,
                position: [node.position.x, node.position.y],
                parameters: node.config.clone(),
            }
        })
        .collect();

    let edges_list: Vec<Edge> = serde_json::from_value(workflow.edges).unwrap_or_default();
    let mut connections = serde_json::Map::new();
    for edge in &edges_list {
        if let Some(source_node) = nodes_list.iter().find(|n| n.id == edge.from) {
            if let Some(_target_node) = nodes_list.iter().find(|n| n.id == edge.to) {
                let source_name = source_node.label.clone();
                if !connections.contains_key(&source_name) {
                    connections.insert(source_name.clone(), serde_json::json!({"main": [[]]}));
                }
            }
        }
    }

    Ok(Json(N8nWorkflowExport {
        name: workflow.name.clone(),
        nodes: n8n_nodes,
        connections: serde_json::Value::Object(connections),
        settings: serde_json::json!({}),
        active: false,
    }))
}

pub async fn export_current_workflow_n8n(
    State(_state): State<AppState>,
) -> Result<Json<N8nWorkflowExport>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn execute_workflow(
    State(state): State<AppState>,
    Json(request): Json<ExecuteWorkflowRequest>,
) -> Result<Json<ExecuteWorkflowResponse>, StatusCode> {
    let execution_id = Uuid::new_v4();
    let start_time = chrono::Utc::now();
    let mut results = Vec::new();
    let mut success = true;

    let workflow_name = sqlx::query_scalar::<_, String>("SELECT name FROM workflows WHERE id = $1")
        .bind(Uuid::parse_str(&request.workflow_id).unwrap_or_default())
        .fetch_optional(&state.db)
        .await
        .unwrap_or_default()
        .unwrap_or_else(|| "Manual Execution".to_string());

    // Use a queue for BFS-like execution to handle branching
    let mut execution_queue = std::collections::VecDeque::new();
    
    // Initial triggers
    if let Some(trigger_id) = request.trigger_node_id {
        if let Some(trigger) = request.nodes.iter().find(|n| n.id == trigger_id) {
            execution_queue.push_back((trigger, serde_json::json!({})));
        }
    } else {
        let trigger_nodes: Vec<&Node> = request.nodes.iter()
            .filter(|n| !request.edges.iter().any(|e| e.to == n.id))
            .collect();
        for trigger in trigger_nodes {
            execution_queue.push_back((trigger, serde_json::json!({})));
        }
    }

    while let Some((node, input_data)) = execution_queue.pop_front() {
        let node_start_time = std::time::Instant::now();
        match execute_single_node(&state.db, node, &request.nodes, &request.edges, &input_data).await {
            Ok(output) => {
                // 1. Handle Filtering
                if let Some(true) = output.get("__filtered").and_then(|v| v.as_bool()) {
                    results.push(NodeExecutionResult {
                        node_id: node.id.to_string(),
                        node_name: node.label.clone(),
                        success: true,
                        output: Some(output),
                        error: None,
                        execution_time_ms: node_start_time.elapsed().as_millis() as u64,
                    });
                    continue; // Stop this branch
                }

                // 2. Handle Wait Signal
                if let Some(true) = output.get("__wait").and_then(|v| v.as_bool()) {
                    // ... (snapshot logic)
                    let snapshot = serde_json::json!({
                        "last_output": input_data,
                        "remaining_queue": execution_queue.iter().map(|(n, i)| (n.id, i)).collect::<Vec<_>>(),
                        "wait_info": output,
                        "current_node_id": node.id
                    });

                    let record = ExecutionRecord {
                        id: execution_id,
                        workflow_id: request.workflow_id.clone(),
                        workflow_name: workflow_name.clone(),
                        start_time,
                        end_time: None,
                        status: "waiting".to_string(),
                        results: serde_json::to_value(&results).unwrap_or(serde_json::json!([])),
                        snapshot: Some(snapshot),
                    };

                    let _ = sqlx::query(
                        "INSERT INTO executions (id, workflow_id, workflow_name, start_time, status, results, snapshot) VALUES ($1, $2, $3, $4, $5, $6, $7)"
                    )
                    .bind(record.id).bind(&record.workflow_id).bind(&record.workflow_name).bind(record.start_time).bind(&record.status).bind(&record.results).bind(record.snapshot).execute(&state.db).await;

                    return Ok(Json(ExecuteWorkflowResponse { success: true, execution_id, results, error: Some("Workflow paused".to_string()) }));
                }

                results.push(NodeExecutionResult {
                    node_id: node.id.to_string(),
                    node_name: node.label.clone(),
                    success: true,
                    output: Some(output.clone()),
                    error: None,
                    execution_time_ms: node_start_time.elapsed().as_millis() as u64,
                });

                // 3. Determine Next Nodes based on port
                let port = output.get("__port").and_then(|v| v.as_str());
                
                let next_edges: Vec<&Edge> = request.edges.iter()
                    .filter(|e| e.from == node.id && (port.is_none() || e.from_port == port.map(|s| s.to_string())))
                    .collect();

                for edge in next_edges {
                    if let Some(next_node) = request.nodes.iter().find(|n| n.id == edge.to) {
                        execution_queue.push_back((next_node, output.clone()));
                    }
                }
            }
            Err(e) => {
                // ... error handling
                success = false;
                results.push(NodeExecutionResult {
                    node_id: node.id.to_string(),
                    node_name: node.label.clone(),
                    success: false,
                    output: None,
                    error: Some(e),
                    execution_time_ms: node_start_time.elapsed().as_millis() as u64,
                });
                break;
            }
        }
    }

    let record = ExecutionRecord {
        id: execution_id,
        workflow_id: request.workflow_id,
        workflow_name,
        start_time,
        end_time: Some(chrono::Utc::now()),
        status: if success { "success".to_string() } else { "failed".to_string() },
        results: serde_json::to_value(&results).unwrap_or(serde_json::json!([])),
        snapshot: None,
    };

    let _ = sqlx::query(
        "INSERT INTO executions (id, workflow_id, workflow_name, start_time, end_time, status, results) VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(record.id).bind(&record.workflow_id).bind(&record.workflow_name).bind(record.start_time).bind(record.end_time).bind(&record.status).bind(&record.results).execute(&state.db).await;

    Ok(Json(ExecuteWorkflowResponse { success, execution_id, results, error: if success { None } else { Some("Workflow execution failed".to_string()) } }))
}

pub async fn list_executions(State(state): State<AppState>) -> Json<Vec<ExecutionRecord>> {
    let executions = sqlx::query_as::<_, ExecutionRecord>("SELECT * FROM executions ORDER BY start_time DESC LIMIT 100").fetch_all(&state.db).await.unwrap_or_default();
    Json(executions)
}

pub async fn get_execution(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<ExecutionRecord>, StatusCode> {
    let record = sqlx::query_as::<_, ExecutionRecord>("SELECT * FROM executions WHERE id = $1").bind(id).fetch_optional(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(record))
}

#[derive(Debug, Deserialize)]
pub struct NodeExecuteRequest { pub node: Node }

pub async fn execute_node_handler(State(state): State<AppState>, Json(request): Json<NodeExecuteRequest>) -> Result<Json<NodeExecutionResult>, StatusCode> {
    let node = &request.node;
    let node_start_time = std::time::Instant::now();

    let result = match execute_single_node(&state.db, node, &vec![], &vec![], &serde_json::json!({})).await {
        Ok(output) => NodeExecutionResult { node_id: node.id.to_string(), node_name: node.label.clone(), success: true, output: Some(output), error: None, execution_time_ms: node_start_time.elapsed().as_millis() as u64 },
        Err(e) => NodeExecutionResult { node_id: node.id.to_string(), node_name: node.label.clone(), success: false, output: None, error: Some(e), execution_time_ms: node_start_time.elapsed().as_millis() as u64 },
    };

    Ok(Json(result))
}

pub async fn list_workflows(State(state): State<AppState>) -> Json<Vec<Workflow>> {
    let workflows = sqlx::query_as::<_, Workflow>("SELECT * FROM workflows").fetch_all(&state.db).await.unwrap_or_default();
    Json(workflows)
}

pub async fn get_workflow(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<Json<Workflow>, StatusCode> {
    let workflow = sqlx::query_as::<_, Workflow>("SELECT * FROM workflows WHERE id = $1").bind(id).fetch_optional(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(workflow))
}

pub async fn create_workflow(State(state): State<AppState>, Json(input): Json<WorkflowInput>) -> Json<Workflow> {
    let workflow = Workflow {
        id: Uuid::new_v4(),
        name: input.name,
        description: input.description,
        nodes: serde_json::to_value(input.nodes).unwrap_or(serde_json::json!([])),
        edges: serde_json::to_value(input.edges).unwrap_or(serde_json::json!([])),
    };
    let _ = sqlx::query("INSERT INTO workflows (id, name, description, nodes, edges) VALUES ($1, $2, $3, $4, $5)").bind(workflow.id).bind(&workflow.name).bind(&workflow.description).bind(&workflow.nodes).bind(&workflow.edges).execute(&state.db).await;
    Json(workflow)
}

pub async fn update_workflow(Path(id): Path<Uuid>, State(state): State<AppState>, Json(input): Json<WorkflowInput>) -> Result<Json<Workflow>, StatusCode> {
    let nodes = serde_json::to_value(input.nodes).unwrap_or(serde_json::json!([]));
    let edges = serde_json::to_value(input.edges).unwrap_or(serde_json::json!([]));
    let result = sqlx::query("UPDATE workflows SET name = $1, description = $2, nodes = $3, edges = $4, updated_at = NOW() WHERE id = $5").bind(&input.name).bind(&input.description).bind(&nodes).bind(&edges).bind(id).execute(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if result.rows_affected() == 0 { return Err(StatusCode::NOT_FOUND); }
    Ok(Json(Workflow { id, name: input.name, description: input.description, nodes, edges }))
}

pub async fn list_credentials(State(state): State<AppState>) -> Json<Vec<Credential>> {
    let credentials = sqlx::query_as::<_, Credential>("SELECT * FROM credentials").fetch_all(&state.db).await.unwrap_or_default();
    Json(credentials)
}

pub async fn create_credential(State(state): State<AppState>, Json(input): Json<CredentialInput>) -> Json<Credential> {
    let credential = Credential { id: Uuid::new_v4(), name: input.name, provider: input.provider, data: input.data, created_at: chrono::Utc::now() };
    let _ = sqlx::query("INSERT INTO credentials (id, name, provider, data, created_at) VALUES ($1, $2, $3, $4, $5)").bind(credential.id).bind(&credential.name).bind(&credential.provider).bind(&credential.data).bind(credential.created_at).execute(&state.db).await;
    Json(credential)
}

pub async fn delete_credential(Path(id): Path<Uuid>, State(state): State<AppState>) -> StatusCode {
    let result = sqlx::query("DELETE FROM credentials WHERE id = $1").bind(id).execute(&state.db).await;
    match result { Ok(res) if res.rows_affected() > 0 => StatusCode::OK, _ => StatusCode::NOT_FOUND }
}

#[derive(Debug, Deserialize)]
pub struct PostgresTestRequest {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub database: String,
}

pub async fn test_postgres_connection(
    Json(req): Json<PostgresTestRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let password = req.password.unwrap_or_default();
    let connection_url = format!("postgres://{}:{}@{}:{}/{}", req.user, password, req.host, req.port, req.database);
    
    match sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&connection_url)
        .await {
            Ok(_) => Ok(Json(serde_json::json!({ "success": true, "message": "Connection successful" }))),
            Err(e) => Ok(Json(serde_json::json!({ "success": false, "message": e.to_string() })))
        }
}

// MCP Server Handlers
pub async fn list_mcp_servers(State(state): State<AppState>) -> Json<Vec<McpServer>> {
    let servers = sqlx::query_as::<_, McpServer>("SELECT * FROM mcp_servers")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    Json(servers)
}

pub async fn create_mcp_server(
    State(state): State<AppState>,
    Json(input): Json<McpServerInput>,
) -> Result<Json<McpServer>, StatusCode> {
    let server = McpServer {
        id: Uuid::new_v4(),
        name: input.name,
        transport: input.transport,
        command: input.command,
        args: serde_json::to_value(input.args).unwrap_or(serde_json::json!([])),
        endpoint: input.endpoint,
        env: serde_json::to_value(input.env).unwrap_or(serde_json::json!([])),
        headers: input.headers.unwrap_or(serde_json::json!({})),
        auto_start: input.auto_start,
        status: "disconnected".to_string(),
    };
    let _ = sqlx::query("INSERT INTO mcp_servers (id, name, transport, command, args, endpoint, env, headers, auto_start, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .bind(server.id).bind(&server.name).bind(&server.transport).bind(&server.command).bind(&server.args).bind(&server.endpoint).bind(&server.env).bind(&server.headers).bind(server.auto_start).bind(&server.status)
        .execute(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(server))
}

pub async fn delete_mcp_server(Path(id): Path<Uuid>, State(state): State<AppState>) -> StatusCode {
    let _ = sqlx::query("DELETE FROM mcp_servers WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await;
    StatusCode::OK
}

pub async fn check_mcp_server_status(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let server = sqlx::query_as::<_, McpServer>("SELECT * FROM mcp_servers WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if server.transport == "streamable-http" {
        if let Some(url) = server.endpoint {
            use rmcp::{ServiceExt, transport::StreamableHttpClientTransport};
            use rmcp::transport::streamable_http_client::StreamableHttpClientTransportConfig;
            use rmcp::model::{ClientCapabilities, ClientInfo, Implementation};
            use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

            // Build custom headers
            let mut headers = HeaderMap::new();
            if let Some(h_obj) = server.headers.as_object() {
                for (k, v) in h_obj {
                    if let Some(val) = v.as_str() {
                        if let (Ok(name), Ok(value)) = (HeaderName::from_bytes(k.as_bytes()), HeaderValue::from_str(val)) {
                            headers.insert(name, value);
                        }
                    }
                }
            }

            let http_client = reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let config = StreamableHttpClientTransportConfig::with_uri(url.clone());
            let transport = StreamableHttpClientTransport::with_client(http_client, config);
            
            let client_info = ClientInfo {
                meta: None,
                protocol_version: Default::default(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "nexus-core".to_string(),
                    title: None,
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    website_url: None,
                    icons: None,
                },
            };

            let client_result = client_info.serve(transport).await;

            match client_result {
                Ok(client) => {
                    let tools_result = client.peer().list_all_tools().await;
                    let _ = client.cancel().await;

                    match tools_result {
                        Ok(tools) => {
                            let _ = sqlx::query("UPDATE mcp_servers SET status = $1 WHERE id = $2")
                                .bind("connected")
                                .bind(id)
                                .execute(&state.db)
                                .await;

                            return Ok(Json(serde_json::json!({
                                "status": "connected",
                                "tools": tools
                            })));
                        }
                        Err(e) => {
                            let _ = sqlx::query("UPDATE mcp_servers SET status = $1 WHERE id = $2")
                                .bind("error")
                                .bind(id)
                                .execute(&state.db)
                                .await;

                            return Ok(Json(serde_json::json!({
                                "status": "error",
                                "error": e.to_string()
                            })));
                        }
                    }
                }
                Err(e) => {
                    let _ = sqlx::query("UPDATE mcp_servers SET status = $1 WHERE id = $2")
                        .bind("error")
                        .bind(id)
                        .execute(&state.db)
                        .await;

                    return Ok(Json(serde_json::json!({
                        "status": "error",
                        "error": e.to_string()
                    })));
                }
            }
        }
    }

    Ok(Json(serde_json::json!({
        "status": "unsupported_transport",
        "tools": []
    })))
}

pub async fn list_mcp_tools(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // For now, we reuse check_mcp_server_status but we could cache this in DB later
    check_mcp_server_status(Path(id), State(state)).await
}

// Dedicated Execution Handlers
#[derive(Debug, Deserialize)]
pub struct LlmExecuteRequest { pub prompt: String, pub model: String }
#[derive(Debug, Serialize)]
pub struct LlmExecuteResponse { pub success: bool, pub output: Option<serde_json::Value>, pub error: Option<String> }

pub async fn execute_llm(State(state): State<AppState>, Json(request): Json<LlmExecuteRequest>) -> Result<Json<LlmExecuteResponse>, StatusCode> {
    let Some(openrouter) = &state.openrouter else {
        return Ok(Json(LlmExecuteResponse { success: false, output: None, error: Some("OpenRouter API key not configured".to_string()) }));
    };
    let messages = vec![OpenRouterMessage { 
        role: "user".to_string(), 
        content: request.prompt,
        tool_calls: None,
        tool_call_id: None
    }];
    let or_request = OpenRouterRequest { 
        model: request.model, 
        messages, 
        temperature: None, 
        max_tokens: None, 
        top_p: None, 
        frequency_penalty: None, 
        presence_penalty: None, 
        response_format: None,
        tools: None,
        tool_choice: None,
    };
    match openrouter.generate(or_request).await {
        Ok(res) => Ok(Json(LlmExecuteResponse { success: true, output: Some(res), error: None })),
        Err(e) => Ok(Json(LlmExecuteResponse { success: false, output: None, error: Some(format!("OpenRouter API error: {}", e)) })),
    }
}

pub async fn execute_openai(State(state): State<AppState>, Json(request): Json<LlmExecuteRequest>) -> Result<Json<LlmExecuteResponse>, StatusCode> {
    let Some(openai) = &state.openai else {
        return Ok(Json(LlmExecuteResponse { success: false, output: None, error: Some("OpenAI API key not configured".to_string()) }));
    };
    let messages = vec![OpenAiMessage { 
        role: "user".to_string(), 
        content: request.prompt,
        tool_calls: None,
        tool_call_id: None
    }];
    match openai.generate(&request.model, messages, None, None, None).await {
        Ok(res) => Ok(Json(LlmExecuteResponse { success: true, output: Some(res), error: None })),
        Err(e) => Ok(Json(LlmExecuteResponse { success: false, output: None, error: Some(format!("OpenAI API error: {}", e)) })),
    }
}

pub async fn chat_trigger(State(_state): State<AppState>, Json(_input): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({ "status": "chat triggered" })))
}

pub async fn execute_http_request(State(_state): State<AppState>, Json(request): Json<HttpRequestExecuteRequest>) -> Result<Json<HttpRequestExecuteResponse>, StatusCode> {
    let client = reqwest::Client::new();
    let method = match request.method.to_uppercase().as_str() {
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => reqwest::Method::GET,
    };
    let mut builder = client.request(method, &request.url);
    if let Some(headers) = request.headers {
        for (k, v) in headers { builder = builder.header(k, v); }
    }
    if let Some(body) = request.body { builder = builder.body(body); }
    match builder.send().await {
        Ok(response) => {
            let status = response.status().as_u16();
            let headers = response.headers().iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string())).collect::<Vec<_>>();
            match response.text().await {
                Ok(body) => Ok(Json(HttpRequestExecuteResponse { success: true, status_code: Some(status), headers: Some(headers), body: Some(body), error: None })),
                Err(e) => Ok(Json(HttpRequestExecuteResponse { success: false, status_code: Some(status), headers: Some(headers), body: None, error: Some(e.to_string()) })),
            }
        }
        Err(e) => Ok(Json(HttpRequestExecuteResponse { success: false, status_code: None, headers: None, body: None, error: Some(e.to_string()) })),
    }
}

#[derive(Debug, Serialize)]
pub struct ExecuteWorkflowResponse { pub success: bool, pub execution_id: Uuid, pub results: Vec<NodeExecutionResult>, pub error: Option<String> }
#[derive(Debug, Deserialize)]
pub struct ExecuteWorkflowRequest { 
    pub workflow_id: String, 
    pub nodes: Vec<Node>, 
    pub edges: Vec<Edge>,
    pub trigger_node_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct HttpRequestExecuteRequest { pub url: String, pub method: String, pub headers: Option<Vec<(String, String)>>, pub body: Option<String> }
#[derive(Debug, Serialize)]
pub struct HttpRequestExecuteResponse { pub success: bool, pub status_code: Option<u16>, pub headers: Option<Vec<(String, String)>>, pub body: Option<String>, pub error: Option<String> }
