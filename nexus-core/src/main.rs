mod models;
mod state;
mod db;
mod handlers;
mod clients;
mod engine;

use axum::{
    routing::{get, post, delete},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use sqlx::postgres::PgPoolOptions;
use crate::state::AppState;
use crate::clients::{OpenAiClient, OpenRouterClient};
use crate::handlers::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let openrouter = std::env::var("OPENROUTER_API_KEY").ok().map(OpenRouterClient::new);
    let openai = std::env::var("OPENAI_API_KEY").ok().map(OpenAiClient::new);

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://nexus:nexus@db:5432/nexus".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    db::init_db(&pool).await.expect("Failed to initialize database");

    let state = AppState {
        db: pool,
        openrouter,
        openai,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/workflows", get(list_workflows).post(create_workflow))
        .route("/api/workflows/:id", get(get_workflow).put(update_workflow))
        .route("/api/mcp/servers", get(list_mcp_servers).post(create_mcp_server))
        .route("/api/mcp/servers/:id/tools", get(list_mcp_tools))
        .route("/api/mcp/servers/:id/status", get(check_mcp_server_status))
        .route("/api/credentials", get(list_credentials).post(create_credential))
        .route("/api/credentials/:id", delete(delete_credential))
        .route("/api/credentials/test/postgres", post(test_postgres_connection))
        .route("/api/llm/execute", post(execute_llm))
        .route("/api/openai/execute", post(execute_openai))
        .route("/api/chat/trigger", post(chat_trigger))
        .route("/api/http-request/execute", post(execute_http_request))
        .route("/api/nodes/execute", post(execute_node_handler))
        .route("/api/workflows/execute", post(execute_workflow))
        .route("/api/workflows/:id/export/n8n", get(export_workflow_n8n))
        .route("/api/workflows/current/export/n8n", get(export_current_workflow_n8n))
        .route("/api/mcp/servers/:id", delete(delete_mcp_server))
        .route("/api/executions", get(list_executions))
        .route("/api/executions/:id", get(get_execution))
        .route("/api/webhooks/slack/interactive", post(handle_slack_interactive))
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    let listener = tokio::net::TcpListener::bind(addr).await.expect("failed to bind");
    println!("nexus-core listening on {}", addr);
    axum::serve(listener, app).await.expect("server error");
}