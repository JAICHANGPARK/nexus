pub mod executor;

use crate::models::{Node, Edge, Credential};
use crate::clients::{OpenAiClient, OpenRouterClient};
use crate::clients::openai::OpenAiMessage;
use crate::clients::openrouter::{OpenRouterMessage, OpenRouterRequest};
use crate::engine::executor::CodeExecutor;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use std::collections::HashSet;

pub fn execute_node_recursive(
    node: &Node,
    all_nodes: &[Node],
    edges: &[Edge],
    visited: &mut HashSet<Uuid>,
    execution_order: &mut Vec<Node>,
) {
    if visited.contains(&node.id) {
        return;
    }

    visited.insert(node.id);

    let child_edges: Vec<&Edge> = edges.iter().filter(|e| e.from == node.id).collect();

    for edge in child_edges {
        if let Some(child_node) = all_nodes.iter().find(|n| n.id == edge.to) {
            execute_node_recursive(child_node, all_nodes, edges, visited, execution_order);
        }
    }

    execution_order.push(node.clone());
}

fn interpolate_value(value: &str, input: &serde_json::Value) -> String {
    let mut result = value.to_string();
    if let Some(obj) = input.as_object() {
        for (k, v) in obj {
            let placeholder = format!("{{{{ $input.{} }}}}", k);
            let replacement = match v {
                serde_json::Value::String(s) => s.clone(),
                _ => v.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
    }
    if input.is_string() {
        result = result.replace("{{ $input }}", input.as_str().unwrap());
    } else {
        result = result.replace("{{ $input }}", &input.to_string());
    }
    result
}

pub async fn execute_single_node(
    pool: &Pool<Postgres>, 
    node: &Node, 
    all_nodes: &[Node], 
    edges: &[Edge],
    input: &serde_json::Value
) -> Result<serde_json::Value, String> {
    match node.kind.as_str() {
        "http-request" => {
            let url_raw = node.config.get("url").and_then(|v| v.as_str()).ok_or("URL not specified")?;
            let url = interpolate_value(url_raw, input);
            let method_str = node.config.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
            let method = match method_str.to_uppercase().as_str() {
                "POST" => reqwest::Method::POST,
                "PUT" => reqwest::Method::PUT,
                "DELETE" => reqwest::Method::DELETE,
                "PATCH" => reqwest::Method::PATCH,
                "HEAD" => reqwest::Method::HEAD,
                "OPTIONS" => reqwest::Method::OPTIONS,
                _ => reqwest::Method::GET,
            };
            let client = reqwest::Client::new();
            let mut req_builder = client.request(method, url);
            if let Some(auth_type) = node.config.get("authentication").and_then(|v| v.as_str()) {
                if auth_type == "basicAuth" {
                    let user = node.config.get("user").and_then(|v| v.as_str()).unwrap_or("");
                    let password = node.config.get("password").and_then(|v| v.as_str()).unwrap_or("");
                    req_builder = req_builder.basic_auth(user, Some(password));
                }
            }
            match req_builder.send().await {
                Ok(response) => {
                    let status = response.status().as_u16();
                    let body = response.text().await.map_err(|e| e.to_string())?;
                    let body_json = serde_json::from_str::<serde_json::Value>(&body).unwrap_or(serde_json::json!(body));
                    let full_response = node.config.get("fullResponse").and_then(|v| v.as_bool()).unwrap_or(false);
                    if full_response { Ok(serde_json::json!({ "status_code": status, "body": body_json })) } else { Ok(body_json) }
                }
                Err(e) => Err(format!("HTTP Error: {}", e)),
            }
        }
        "openai" => {
            let api_key = get_api_key(pool, node, "openai", "OPENAI_API_KEY").await?;
            let client = OpenAiClient::new(api_key);
            let resource = node.config.get("resource").and_then(|v| v.as_str()).unwrap_or("chat");
            let operation = node.config.get("operation").and_then(|v| v.as_str()).unwrap_or("completions");
            match (resource, operation) {
                ("chat", "completions") => {
                    let model = node.config.get("model").and_then(|v| v.as_str()).unwrap_or("gpt-4o");
                    let prompt_raw = node.config.get("prompt").and_then(|v| v.as_str()).ok_or("Prompt not specified")?;
                    let prompt = interpolate_value(prompt_raw, input);
                    let system = node.config.get("systemMessage").and_then(|v| v.as_str());
                    let mut messages = Vec::new();
                    if let Some(s) = system { messages.push(OpenAiMessage { role: "system".to_string(), content: s.to_string(), tool_calls: None, tool_call_id: None }); }
                    messages.push(OpenAiMessage { role: "user".to_string(), content: prompt.to_string(), tool_calls: None, tool_call_id: None });
                    let result = client.generate(model, messages, None, None, None).await.map_err(|e| e.to_string())?;
                    Ok(result)
                },
                ("image", "generate") => {
                    let prompt_raw = node.config.get("prompt").and_then(|v| v.as_str()).ok_or("Prompt not specified")?;
                    let prompt = interpolate_value(prompt_raw, input);
                    let model = node.config.get("model").and_then(|v| v.as_str()).unwrap_or("dall-e-3");
                    let size = node.config.get("size").and_then(|v| v.as_str()).unwrap_or("1024x1024");
                    client.images_generate(&prompt, model, size, "standard", 1).await.map_err(|e| e.to_string())
                },
                _ => Err("Unsupported OpenAI operation".to_string())
            }
        }
        "openrouter" => {
            let api_key = get_api_key(pool, node, "openrouter", "OPENROUTER_API_KEY").await?;
            let client = OpenRouterClient::new(api_key);
            let model = node.config.get("model").and_then(|v| v.as_str()).unwrap_or("openai/gpt-4o-mini");
            let prompt_raw = node.config.get("prompt").and_then(|v| v.as_str()).ok_or("Prompt not specified")?;
            let prompt = interpolate_value(prompt_raw, input);
            let system = node.config.get("systemMessage").and_then(|v| v.as_str());
            let mut messages = Vec::new();
            if let Some(s) = system { messages.push(OpenRouterMessage { role: "system".to_string(), content: s.to_string(), tool_calls: None, tool_call_id: None }); }
            messages.push(OpenRouterMessage { role: "user".to_string(), content: prompt.to_string(), tool_calls: None, tool_call_id: None });
            let request = OpenRouterRequest {
                model: model.to_string(),
                messages,
                temperature: node.config.get("temperature").and_then(|v| v.as_f64()).map(|v| v as f32),
                max_tokens: node.config.get("maxTokens").and_then(|v| v.as_i64()).map(|v| v as i32),
                top_p: node.config.get("topP").and_then(|v| v.as_f64()).map(|v| v as f32),
                frequency_penalty: node.config.get("frequencyPenalty").and_then(|v| v.as_f64()).map(|v| v as f32),
                presence_penalty: node.config.get("presencePenalty").and_then(|v| v.as_f64()).map(|v| v as f32),
                response_format: None,
                tools: None,
                tool_choice: None,
            };
            let result = client.generate(request).await.map_err(|e| e.to_string())?;
            Ok(result)
        }
        "ai-agent" => {
            execute_agent(pool, node, all_nodes, edges, input).await
        }
        "llm" => {
            let api_key = get_api_key(pool, node, "openrouter", "OPENROUTER_API_KEY").await?;
            let client = OpenRouterClient::new(api_key);
            let model = node.config.get("model").and_then(|v| v.as_str()).unwrap_or("openai/gpt-4o-mini");
            let prompt_raw = node.config.get("prompt").and_then(|v| v.as_str()).ok_or("Prompt not specified")?;
            let prompt = interpolate_value(prompt_raw, input);
            let messages = vec![OpenRouterMessage { role: "user".to_string(), content: prompt, tool_calls: None, tool_call_id: None }];
            let request = OpenRouterRequest {
                model: model.to_string(),
                messages,
                temperature: Some(0.7),
                max_tokens: Some(1000),
                top_p: None,
                frequency_penalty: None,
                presence_penalty: None,
                response_format: None,
                tools: None,
                tool_choice: None,
            };
            let result = client.generate(request).await.map_err(|e| e.to_string())?;
            Ok(result)
        }
        "tool" => Ok(node.config.clone()),
        "code" => {
            let code = node.config.get("code").and_then(|v| v.as_str()).unwrap_or("return $input;");
            let language = node.config.get("language").and_then(|v| v.as_str()).unwrap_or("javascript");
            if language == "javascript" { CodeExecutor::execute_js(code, input).await.map_err(|e| format!("JS Error: {}", e)) }
            else if language == "python" { CodeExecutor::execute_python(code, input).await.map_err(|e| format!("Python Error: {}", e)) }
            else { Err(format!("Unsupported language: {}", language)) }
        }
        "wait" => {
            let amount = node.config.get("amount").and_then(|v| v.as_f64()).unwrap_or(1.0);
            let unit = node.config.get("unit").and_then(|v| v.as_str()).unwrap_or("seconds");
            let seconds = match unit { "minutes" => amount * 60.0, "hours" => amount * 3600.0, _ => amount };
            tokio::time::sleep(std::time::Duration::from_secs_f64(seconds)).await;
            Ok(serde_json::json!({ "waited": seconds, "unit": "seconds" }))
        }
        "trigger-start" | "trigger-schedule" | "trigger-webhook" => Ok(serde_json::json!({ "triggered": true })),
        "chat-trigger" => Ok(node.config.get("initialInput").cloned().unwrap_or(serde_json::json!({ "triggered": true }))),
        _ => Ok(serde_json::json!({ "result": "Node executed" })),
    }
}

async fn get_api_key(pool: &Pool<Postgres>, node: &Node, _provider: &str, env_var: &str) -> Result<String, String> {
    if let Some(cred_id) = node.config.get("credentialId").and_then(|v| v.as_str()) {
        if let Ok(id) = Uuid::parse_str(cred_id) {
            let cred = sqlx::query_as::<_, Credential>("SELECT * FROM credentials WHERE id = $1").bind(id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
            if let Some(c) = cred { return c.data.get("api_key").and_then(|v| v.as_str()).map(|s| s.to_string()).ok_or("API key not found in credential".to_string()); }
        }
    }
    std::env::var(env_var).map_err(|_| format!("{} not set", env_var))
}

async fn execute_agent(pool: &Pool<Postgres>, node: &Node, all_nodes: &[Node], edges: &[Edge], input: &serde_json::Value) -> Result<serde_json::Value, String> {
    let provider = node.config.get("provider").and_then(|v| v.as_str()).unwrap_or("openai");
    let model = node.config.get("model").and_then(|v| v.as_str()).unwrap_or("gpt-4o");
    let prompt_raw = node.config.get("prompt").and_then(|v| v.as_str()).ok_or("Prompt not specified")?;
    let prompt = interpolate_value(prompt_raw, input);
    let system_message_raw = node.config.get("systemMessage").and_then(|v| v.as_str());
    let system_message = system_message_raw.map(|s| interpolate_value(s, input));

    let tool_nodes: Vec<&Node> = edges.iter().filter(|e| e.to == node.id && e.to_port == Some("tools".to_string())).filter_map(|e| all_nodes.iter().find(|n| n.id == e.from)).filter(|n| n.kind == "tool").collect();
    let mut tools_schema = Vec::new();
    for tool_node in &tool_nodes {
        let tool_name = tool_node.config.get("toolName").and_then(|v| v.as_str()).unwrap_or("unknown_tool");
        tools_schema.push(serde_json::json!({ "type": "function", "function": { "name": tool_name, "description": format!("Executes the {} tool", tool_name), "parameters": { "type": "object", "properties": { "query": { "type": "string" } } } } }));
    }
    let api_key = if provider == "openai" { get_api_key(pool, node, "openai", "OPENAI_API_KEY").await? } else { get_api_key(pool, node, "openrouter", "OPENROUTER_API_KEY").await? };
    let mut current_messages = Vec::new();
    if let Some(s) = system_message { current_messages.push(OpenAiMessage { role: "system".to_string(), content: s, tool_calls: None, tool_call_id: None }); }
    current_messages.push(OpenAiMessage { role: "user".to_string(), content: prompt, tool_calls: None, tool_call_id: None });
    let tools_value = if !tools_schema.is_empty() { Some(serde_json::Value::Array(tools_schema)) } else { None };
    for _ in 0..5 {
        let response = if provider == "openai" {
            let client = OpenAiClient::new(api_key.clone());
            client.generate(model, current_messages.clone(), None, None, tools_value.clone()).await.map_err(|e| e.to_string())?
        } else {
            let client = OpenRouterClient::new(api_key.clone());
            let or_messages: Vec<OpenRouterMessage> = current_messages.iter().map(|m| OpenRouterMessage { role: m.role.clone(), content: m.content.clone(), tool_calls: m.tool_calls.clone(), tool_call_id: m.tool_call_id.clone() }).collect();
            let or_request = OpenRouterRequest { model: model.to_string(), messages: or_messages, temperature: None, max_tokens: None, top_p: None, frequency_penalty: None, presence_penalty: None, response_format: None, tools: tools_value.clone(), tool_choice: None };
            client.generate(or_request).await.map_err(|e| e.to_string())?
        };
        let message = response.get("choices").and_then(|c| c.get(0)).and_then(|m| m.get("message")).ok_or("Invalid LLM response")?;
        if let Some(tool_calls) = message.get("tool_calls") {
            current_messages.push(OpenAiMessage { role: "assistant".to_string(), content: message.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(), tool_calls: Some(tool_calls.clone()), tool_call_id: None });
            for call in tool_calls.as_array().unwrap_or(&vec![]) {
                let call_id = call.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let func_name = call.get("function").and_then(|f| f.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                let tool_node = tool_nodes.iter().find(|tn| tn.config.get("toolName").and_then(|v| v.as_str()) == Some(func_name));
                let tool_result = if let Some(_tn) = tool_node { format!("Result from {}: Action completed successfully.", func_name) } else { format!("Error: Tool '{}' not found", func_name) };
                current_messages.push(OpenAiMessage { role: "tool".to_string(), content: tool_result, tool_calls: None, tool_call_id: Some(call_id.to_string()) });
            }
        } else {
            let text = message.get("content").and_then(|v| v.as_str()).unwrap_or("");
            return Ok(serde_json::json!({ "text": text }));
        }
    }
    Err("Agent reached maximum iterations".to_string())
}