pub mod executor;

use crate::models::{Node, Edge, Credential, McpServer};
use crate::clients::{OpenAiClient, OpenRouterClient};
use crate::clients::openai::OpenAiMessage;
use crate::clients::openrouter::{OpenRouterMessage, OpenRouterRequest};
use crate::engine::executor::CodeExecutor;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

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

lazy_static::lazy_static! {
    static ref EMPTY_OBJECT: serde_json::Value = serde_json::json!({});
}

fn evaluate_conditions(conditions: &serde_json::Value, input: &serde_json::Value, ignore_case: bool) -> bool {
    let Some(cond_list) = conditions.get("conditions").and_then(|v| v.as_array()) else { return true; };
    let combinator = conditions.get("combinator").and_then(|v| v.as_str()).unwrap_or("and");

    let mut results = Vec::new();
    for cond in cond_list {
        let left_raw = cond.get("leftValue").and_then(|v| v.as_str()).unwrap_or("");
        let left = interpolate_value(left_raw, input);
        
        let op_obj = cond.get("operator");
        let op_type = op_obj.and_then(|v| v.get("type")).and_then(|v| v.as_str()).unwrap_or("string");
        let op_name = op_obj.and_then(|v| v.get("operation")).and_then(|v| v.as_str()).unwrap_or("equals");
        
        let right_raw = cond.get("rightValue").and_then(|v| v.as_str()).unwrap_or("");
        let right = interpolate_value(right_raw, input);

        let pass = match op_type {
            "string" => {
                let (l, r): (String, String) = if ignore_case { (left.to_lowercase(), right.to_lowercase()) } else { (left, right) };
                match op_name {
                    "equals" => l == r,
                    "notEquals" => l != r,
                    "contains" => l.contains(&r),
                    "notContains" => !l.contains(&r),
                    "startsWith" => l.starts_with(&r),
                    "endsWith" => l.ends_with(&r),
                    "isEmpty" => l.is_empty(),
                    "isNotEmpty" => !l.is_empty(),
                    _ => false
                }
            },
            "number" => {
                let l = left.parse::<f64>().unwrap_or(0.0);
                let r = right.parse::<f64>().unwrap_or(0.0);
                match op_name {
                    "equals" | "eq" => l == r,
                    "notEquals" | "ne" => l != r,
                    "gt" | "larger" => l > r,
                    "gte" | "largerEqual" => l >= r,
                    "lt" | "smaller" => l < r,
                    "lte" | "smallerEqual" => l <= r,
                    _ => false
                }
            },
            "boolean" => {
                let l = left.parse::<bool>().unwrap_or(false);
                let r = right.parse::<bool>().unwrap_or(false);
                match op_name {
                    "true" => l == true,
                    "false" => l == false,
                    "equals" => l == r,
                    _ => false
                }
            },
            _ => false
        };
        results.push(pass);
    }

    if combinator == "or" {
        results.iter().any(|&r| r)
    } else {
        results.iter().all(|&r| r)
    }
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
        "if" => {
            let conditions = node.config.get("conditions").unwrap_or(&*EMPTY_OBJECT);
            let ignore_case = node.config.get("options").and_then(|v| v.get("ignoreCase")).and_then(|v| v.as_bool()).unwrap_or(true);
            let pass = evaluate_conditions(conditions, input, ignore_case);
            
            let mut output = input.clone();
            if let Some(obj) = output.as_object_mut() {
                obj.insert("__port".to_string(), serde_json::json!(if pass { "true" } else { "false" }));
            }
            Ok(output)
        },
        "filter" => {
            let conditions = node.config.get("conditions").unwrap_or(&*EMPTY_OBJECT);
            let ignore_case = node.config.get("options").and_then(|v| v.get("ignoreCase")).and_then(|v| v.as_bool()).unwrap_or(true);
            let pass = evaluate_conditions(conditions, input, ignore_case);
            
            if pass { Ok(input.clone()) } else { Ok(serde_json::json!({ "__filtered": true })) }
        },
        "switch" => {
            let mode = node.config.get("mode").and_then(|v| v.as_str()).unwrap_or("rules");
            if mode == "rules" {
                let rules = node.config.get("rules").and_then(|v| v.get("values")).and_then(|v| v.as_array());
                let ignore_case = node.config.get("options").and_then(|v| v.get("ignoreCase")).and_then(|v| v.as_bool()).unwrap_or(true);
                
                let mut matched_index = None;
                if let Some(rules) = rules {
                    for (i, rule) in rules.iter().enumerate() {
                        let cond = rule.get("conditions").unwrap_or(&*EMPTY_OBJECT);
                        if evaluate_conditions(cond, input, ignore_case) {
                            matched_index = Some(i);
                            break;
                        }
                    }
                }
                
                let port = if let Some(idx) = matched_index { idx.to_string() } else { "fallback".to_string() };
                let mut output = input.clone();
                if let Some(obj) = output.as_object_mut() {
                    obj.insert("__port".to_string(), serde_json::json!(port));
                }
                Ok(output)
            } else {
                // Expression mode (simplified)
                let port = node.config.get("output").and_then(|v| v.as_i64()).unwrap_or(0);
                let mut output = input.clone();
                if let Some(obj) = output.as_object_mut() {
                    obj.insert("__port".to_string(), serde_json::json!(port.to_string()));
                }
                Ok(output)
            }
        },
        "postgres" => {
            let operation = node.config.get("operation").and_then(|v| v.as_str()).unwrap_or("select");
            let cred_id = node.config.get("credentialId").and_then(|v| v.as_str()).ok_or("Credential not specified")?;
            
            // 1. Fetch Credentials
            let cred = sqlx::query_as::<_, Credential>("SELECT * FROM credentials WHERE id = $1")
                .bind(Uuid::parse_str(cred_id).map_err(|e| e.to_string())?)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?
                .ok_or("Credential not found")?;

            // 2. Build Connection Pool for External DB
            let host = cred.data.get("host").and_then(|v| v.as_str()).unwrap_or("localhost");
            let port = cred.data.get("port").and_then(|v| v.as_u64()).or_else(|| cred.data.get("port").and_then(|v| v.as_str()).and_then(|s| s.parse::<u64>().ok())).unwrap_or(5432);
            let user = cred.data.get("user").and_then(|v| v.as_str()).unwrap_or("postgres");
            let password = cred.data.get("password").and_then(|v| v.as_str()).unwrap_or("");
            let database = cred.data.get("database").and_then(|v| v.as_str()).unwrap_or("postgres");
            
            let connection_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, database);
            let ext_pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(1)
                .connect(&connection_url)
                .await
                .map_err(|e| format!("Failed to connect to external Postgres: {}", e))?;

            // 3. Execute Operation
            let result = match operation {
                "executeQuery" => {
                    let query_raw = node.config.get("query").and_then(|v| v.as_str()).ok_or("Query not specified")?;
                    let query = interpolate_value(query_raw, input);
                    
                    let rows = sqlx::query(&query)
                        .fetch_all(&ext_pool)
                        .await
                        .map_err(|e: sqlx::Error| e.to_string())?;
                    
                    let mut results = Vec::new();
                    for row in rows {
                        use sqlx::Column;
                        use sqlx::Row;
                        let mut map = serde_json::Map::new();
                        for col in row.columns() {
                            let name = col.name();
                            let val: serde_json::Value = if let Ok(v) = row.try_get::<String, _>(name) { serde_json::json!(v) }
                            else if let Ok(v) = row.try_get::<i64, _>(name) { serde_json::json!(v) }
                            else if let Ok(v) = row.try_get::<f64, _>(name) { serde_json::json!(v) }
                            else if let Ok(v) = row.try_get::<bool, _>(name) { serde_json::json!(v) }
                            else { serde_json::json!(null) };
                            map.insert(name.to_string(), val);
                        }
                        results.push(serde_json::Value::Object(map));
                    }
                    serde_json::Value::Array(results)
                },
                "select" => {
                    let schema = node.config.get("schema").and_then(|v| v.as_str()).unwrap_or("public");
                    let table = node.config.get("table").and_then(|v| v.as_str()).ok_or("Table not specified")?;
                    let where_clause = node.config.get("where").and_then(|v| v.as_str()).unwrap_or("");
                    let limit = node.config.get("limit").and_then(|v| v.as_i64()).unwrap_or(50);
                    let sort = node.config.get("sort").and_then(|v| v.as_str()).unwrap_or("");

                    let mut query = format!("SELECT * FROM {}.{}", schema, table);
                    if !where_clause.is_empty() {
                        query = format!("{} WHERE {}", query, where_clause);
                    }
                    if !sort.is_empty() {
                        query = format!("{} ORDER BY {}", query, sort);
                    }
                    query = format!("{} LIMIT {}", query, limit);

                    let rows = sqlx::query(&query).fetch_all(&ext_pool).await.map_err(|e: sqlx::Error| e.to_string())?;
                    let mut results = Vec::new();
                    for row in rows {
                        use sqlx::Column;
                        use sqlx::Row;
                        let mut map = serde_json::Map::new();
                        for col in row.columns() {
                            let name = col.name();
                            let val: serde_json::Value = if let Ok(v) = row.try_get::<String, _>(name) { serde_json::json!(v) }
                            else if let Ok(v) = row.try_get::<i64, _>(name) { serde_json::json!(v) }
                            else if let Ok(v) = row.try_get::<f64, _>(name) { serde_json::json!(v) }
                            else if let Ok(v) = row.try_get::<bool, _>(name) { serde_json::json!(v) }
                            else { serde_json::json!(null) };
                            map.insert(name.to_string(), val);
                        }
                        results.push(serde_json::Value::Object(map));
                    }
                    serde_json::Value::Array(results)
                },
                "insert" => {
                    let schema = node.config.get("schema").and_then(|v| v.as_str()).unwrap_or("public");
                    let table = node.config.get("table").and_then(|v| v.as_str()).ok_or("Table not specified")?;
                    let columns_str = node.config.get("columns").and_then(|v| v.as_str()).ok_or("Columns not specified")?;
                    let columns: Vec<&str> = columns_str.split(',').map(|s| s.trim()).collect();
                    
                    let mut col_names = Vec::new();
                    let mut placeholders = Vec::new();
                    let mut values = Vec::new();
                    
                    for (i, &col) in columns.iter().enumerate() {
                        col_names.push(col);
                        placeholders.push(format!("${}", i + 1));
                        let val = input.get(col).cloned().unwrap_or(serde_json::json!(null));
                        values.push(val);
                    }
                    
                    let query = format!("INSERT INTO {}.{} ({}) VALUES ({}) RETURNING *", schema, table, col_names.join(", "), placeholders.join(", "));
                    
                    let mut q = sqlx::query(&query);
                    for val in values {
                        if let Some(s) = val.as_str() { q = q.bind(s.to_string()); }
                        else if let Some(n) = val.as_i64() { q = q.bind(n); }
                        else if let Some(f) = val.as_f64() { q = q.bind(f); }
                        else if let Some(b) = val.as_bool() { q = q.bind(b); }
                        else { q = q.bind(None::<String>); }
                    }
                    
                    let row = q.fetch_one(&ext_pool).await.map_err(|e: sqlx::Error| e.to_string())?;
                    use sqlx::Column;
                    use sqlx::Row;
                    let mut map = serde_json::Map::new();
                    for col in row.columns() {
                        let name = col.name();
                        let val: serde_json::Value = if let Ok(v) = row.try_get::<String, _>(name) { serde_json::json!(v) }
                        else if let Ok(v) = row.try_get::<i64, _>(name) { serde_json::json!(v) }
                        else { serde_json::json!(null) };
                        map.insert(name.to_string(), val);
                    }
                    serde_json::Value::Object(map)
                },
                _ => serde_json::json!({ "status": "unsupported operation" })
            };

            Ok(result)
        },
        "convert-to-file" => {
            let operation = node.config.get("operation").and_then(|v| v.as_str()).unwrap_or("csv");
            let _binary_property = node.config.get("binaryPropertyName").and_then(|v| v.as_str()).unwrap_or("data");
            
            match operation {
                "csv" => {
                    let mut wtr = csv::Writer::from_writer(vec![]);
                    if let Some(arr) = input.as_array() {
                        for item in arr {
                            if let Some(obj) = item.as_object() {
                                wtr.serialize(obj).map_err(|e| e.to_string())?;
                            }
                        }
                    } else if let Some(obj) = input.as_object() {
                        wtr.serialize(obj).map_err(|e| e.to_string())?;
                    }
                    let data = String::from_utf8(wtr.into_inner().map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
                    Ok(serde_json::json!({ "data": data, "format": "csv" }))
                },
                "toJson" => {
                    let data = serde_json::to_string_pretty(input).map_err(|e| e.to_string())?;
                    Ok(serde_json::json!({ "data": data, "format": "json" }))
                },
                "toText" => {
                    let source = node.config.get("sourceProperty").and_then(|v| v.as_str()).unwrap_or("data");
                    let data = input.get(source).and_then(|v| v.as_str()).unwrap_or("");
                    Ok(serde_json::json!({ "data": data, "format": "text" }))
                },
                "toBinary" => {
                    let source = node.config.get("sourceProperty").and_then(|v| v.as_str()).unwrap_or("data");
                    let b64 = input.get(source).and_then(|v| v.as_str()).unwrap_or("");
                    Ok(serde_json::json!({ "data": b64, "format": "base64" }))
                },
                _ => Err(format!("Unsupported convert operation: {}", operation))
            }
        },
        "extract-from-file" => {
            let operation = node.config.get("operation").and_then(|v| v.as_str()).unwrap_or("csv");
            let source_field = node.config.get("binaryPropertyName").and_then(|v| v.as_str()).unwrap_or("data");
            let content = input.get(source_field).and_then(|v| v.as_str()).ok_or("Source data not found")?;

            match operation {
                "csv" => {
                    let mut rdr = csv::Reader::from_reader(content.as_bytes());
                    let mut results = Vec::new();
                    for result in rdr.deserialize() {
                        let record: serde_json::Value = result.map_err(|e| e.to_string())?;
                        results.push(record);
                    }
                    Ok(serde_json::Value::Array(results))
                },
                "fromJson" => {
                    let val: serde_json::Value = serde_json::from_str(content).map_err(|e| e.to_string())?;
                    Ok(val)
                },
                "text" | "binaryToPropery" => {
                    Ok(serde_json::json!({ "data": content }))
                },
                _ => Err(format!("Unsupported extract operation: {}", operation))
            }
        },
        "read-write-file" => {
            let operation = node.config.get("operation").and_then(|v| v.as_str()).unwrap_or("read");
            
            match operation {
                "read" => {
                    let pattern = node.config.get("fileSelector").and_then(|v| v.as_str()).ok_or("File selector not specified")?;
                    let mut results = Vec::new();
                    for entry in glob::glob(pattern).map_err(|e| e.to_string())? {
                        match entry {
                            Ok(path) => {
                                let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
                                results.push(serde_json::json!({
                                    "path": path.to_string_lossy(),
                                    "data": content
                                }));
                            },
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    Ok(serde_json::Value::Array(results))
                },
                "write" => {
                    let file_path = node.config.get("fileName").and_then(|v| v.as_str()).ok_or("File name not specified")?;
                    let source_field = node.config.get("dataPropertyName").and_then(|v| v.as_str()).unwrap_or("data");
                    let content = input.get(source_field).and_then(|v| v.as_str()).ok_or("Data to write not found")?;
                    
                    use std::io::Write;
                    let mut file = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(node.config.get("append").and_then(|v| v.as_bool()).unwrap_or(false))
                        .open(file_path).map_err(|e| e.to_string())?;
                    
                    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
                    Ok(serde_json::json!({ "success": true, "path": file_path }))
                },
                _ => Err(format!("Unsupported file operation: {}", operation))
            }
        },

        "trigger-start" | "trigger-schedule" | "trigger-webhook" => Ok(serde_json::json!({ "triggered": true })),
        "rss-feed-read" => {
            let url_raw = node.config.get("url").and_then(|v| v.as_str()).ok_or("URL not specified")?;
            let url = interpolate_value(url_raw, input);
            
            let client = reqwest::Client::new();
            let response = client.get(&url).send().await.map_err(|e| format!("Request Error: {}", e))?;
            let content = response.bytes().await.map_err(|e| format!("Byte Error: {}", e))?;
            
            let feed = feed_rs::parser::parse(&content[..]).map_err(|e| format!("Feed Parsing Error: {}", e))?;
            
            let mut items = Vec::new();
            for entry in feed.entries {
                items.push(serde_json::json!({
                    "id": entry.id,
                    "title": entry.title.map(|t| t.content),
                    "link": entry.links.first().map(|l| l.href.clone()),
                    "summary": entry.summary.map(|s| s.content),
                    "content": entry.content.map(|c| c.body.unwrap_or_default()),
                    "published": entry.published,
                    "updated": entry.updated,
                    "author": entry.authors.first().map(|a| a.name.clone()),
                }));
            }
            
            Ok(serde_json::Value::Array(items))
        }
        "slack" => {
            let resource = node.config.get("resource").and_then(|v| v.as_str()).unwrap_or("message");
            let operation = node.config.get("operation").and_then(|v| v.as_str()).unwrap_or("post");
            
            let api_key = get_api_key(pool, node, "slack", "SLACK_TOKEN").await?;
            let client = reqwest::Client::new();
            
            match resource {
                "message" => match operation {
                    "post" | "postEphemeral" | "sendAndWait" => {
                        let channel_raw = node.config.get("channel").and_then(|v| v.as_str())
                            .or_else(|| node.config.get("channelId").and_then(|v| v.as_str()))
                            .ok_or("Channel not specified")?;
                        let channel = interpolate_value(channel_raw, input);
                        
                        let text_raw = node.config.get("text").and_then(|v| v.as_str()).unwrap_or("");
                        let text = interpolate_value(text_raw, input);
                        
                        let mut body = serde_json::json!({
                            "channel": channel,
                            "text": text,
                        });
                        
                        if operation == "postEphemeral" {
                            let user_raw = node.config.get("user").and_then(|v| v.as_str()).ok_or("User not specified for ephemeral message")?;
                            body.as_object_mut().unwrap().insert("user".to_string(), serde_json::json!(interpolate_value(user_raw, input)));
                        }

                        if operation == "sendAndWait" {
                            // Add default Approval buttons if no blocks provided
                            if node.config.get("blocks").is_none() {
                                let approve_text = node.config.get("approveLabel").and_then(|v| v.as_str()).unwrap_or("Approve");
                                let reject_text = node.config.get("rejectLabel").and_then(|v| v.as_str()).unwrap_or("Reject");
                                
                                body.as_object_mut().unwrap().insert("blocks".to_string(), serde_json::json!([
                                    {
                                        "type": "section",
                                        "text": { "type": "mrkdwn", "text": text }
                                    },
                                    {
                                        "type": "actions",
                                        "elements": [
                                            {
                                                "type": "button",
                                                "text": { "type": "plain_text", "text": approve_text },
                                                "style": "primary",
                                                "action_id": "approve"
                                            },
                                            {
                                                "type": "button",
                                                "text": { "type": "plain_text", "text": reject_text },
                                                "style": "danger",
                                                "action_id": "reject"
                                            }
                                        ]
                                    }
                                ]));
                                // Clear top-level text if using blocks for cleaner notification
                                body.as_object_mut().unwrap().insert("text".to_string(), serde_json::json!(approve_text));
                            }
                        }

                        if let Some(blocks) = node.config.get("blocks") {
                            body.as_object_mut().unwrap().insert("blocks".to_string(), blocks.clone());
                        }
                        
                        let endpoint = if operation == "postEphemeral" { "chat.postEphemeral" } else { "chat.postMessage" };
                        let response = client.post(format!("https://slack.com/api/{}", endpoint))
                            .header("Authorization", format!("Bearer {}", api_key))
                            .json(&body).send().await.map_err(|e| e.to_string())?;
                        
                        let res_json = response.json::<serde_json::Value>().await.map_err(|e| e.to_string())?;
                        
                        if res_json.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
                            if operation == "sendAndWait" {
                                // Return a special signal to pause execution
                                return Ok(serde_json::json!({
                                    "__wait": true,
                                    "type": "slack_interactive",
                                    "channel": channel,
                                    "ts": res_json.get("ts")
                                }));
                            }
                            Ok(res_json)
                        }
                        else { Err(format!("Slack API Error: {}", res_json.get("error").and_then(|v| v.as_str()).unwrap_or("Unknown error"))) }
                    },
                    "update" => {
                        let channel_raw = node.config.get("channelId").and_then(|v| v.as_str()).ok_or("Channel not specified")?;
                        let ts = node.config.get("ts").and_then(|v| v.as_str()).ok_or("TS not specified")?;
                        let text_raw = node.config.get("text").and_then(|v| v.as_str()).unwrap_or("");
                        
                        let body = serde_json::json!({
                            "channel": interpolate_value(channel_raw, input),
                            "ts": interpolate_value(ts, input),
                            "text": interpolate_value(text_raw, input),
                        });
                        
                        let response = client.post("https://slack.com/api/chat.update")
                            .header("Authorization", format!("Bearer {}", api_key))
                            .json(&body).send().await.map_err(|e| e.to_string())?;
                        Ok(response.json().await.map_err(|e| e.to_string())?)
                    },
                    "delete" => {
                        let channel_raw = node.config.get("channelId").and_then(|v| v.as_str()).ok_or("Channel not specified")?;
                        let ts = node.config.get("ts").and_then(|v| v.as_str()).ok_or("TS not specified")?;
                        
                        let body = serde_json::json!({
                            "channel": interpolate_value(channel_raw, input),
                            "ts": interpolate_value(ts, input),
                        });
                        
                        let response = client.post("https://slack.com/api/chat.delete")
                            .header("Authorization", format!("Bearer {}", api_key))
                            .json(&body).send().await.map_err(|e| e.to_string())?;
                        Ok(response.json().await.map_err(|e| e.to_string())?)
                    },
                    "search" => {
                        let query = node.config.get("query").and_then(|v| v.as_str()).ok_or("Query not specified")?;
                        let response = client.get("https://slack.com/api/search.messages")
                            .header("Authorization", format!("Bearer {}", api_key))
                            .query(&[("query", interpolate_value(query, input))])
                            .send().await.map_err(|e| e.to_string())?;
                        Ok(response.json().await.map_err(|e| e.to_string())?)
                    },
                    _ => Err(format!("Unsupported Slack operation: {}", operation))
                },
                "channel" => match operation {
                    "create" => {
                        let name = node.config.get("name").and_then(|v| v.as_str()).ok_or("Channel name not specified")?;
                        let is_private = node.config.get("isPrivate").and_then(|v| v.as_bool()).unwrap_or(false);
                        let body = serde_json::json!({ "name": interpolate_value(name, input), "is_private": is_private });
                        let response = client.post("https://slack.com/api/conversations.create")
                            .header("Authorization", format!("Bearer {}", api_key))
                            .json(&body).send().await.map_err(|e| e.to_string())?;
                        Ok(response.json().await.map_err(|e| e.to_string())?)
                    },
                    "getAll" => {
                        let response = client.get("https://slack.com/api/conversations.list")
                            .header("Authorization", format!("Bearer {}", api_key))
                            .send().await.map_err(|e| e.to_string())?;
                        Ok(response.json().await.map_err(|e| e.to_string())?)
                    },
                    _ => Err(format!("Unsupported Slack operation: {}", operation))
                },
                "user" => match operation {
                    "info" => {
                        let user = node.config.get("user").and_then(|v| v.as_str()).ok_or("User ID not specified")?;
                        let response = client.get("https://slack.com/api/users.info")
                            .header("Authorization", format!("Bearer {}", api_key))
                            .query(&[("user", interpolate_value(user, input))])
                            .send().await.map_err(|e| e.to_string())?;
                        Ok(response.json().await.map_err(|e| e.to_string())?)
                    },
                    "getAll" => {
                        let response = client.get("https://slack.com/api/users.list")
                            .header("Authorization", format!("Bearer {}", api_key))
                            .send().await.map_err(|e| e.to_string())?;
                        Ok(response.json().await.map_err(|e| e.to_string())?)
                    },
                    _ => Err(format!("Unsupported Slack operation: {}", operation))
                },
                _ => Err(format!("Unsupported Slack resource: {}", resource))
            }
        }
        "dateTime" => {
            let action = node.config.get("action").and_then(|v| v.as_str()).or_else(|| node.config.get("operation").and_then(|v| v.as_str())).unwrap_or("format");
            let value_raw = node.config.get("value").and_then(|v| v.as_str()).or_else(|| node.config.get("date").and_then(|v| v.as_str())).ok_or("Value not specified")?;
            let value = interpolate_value(value_raw, input);
            
            // Basic parsing using chrono
            let dt = if let Ok(d) = chrono::DateTime::parse_from_rfc3339(&value) {
                d.with_timezone(&chrono::Utc)
            } else if let Ok(d) = chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc)
            } else if let Ok(d) = chrono::NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d.and_hms_opt(0, 0, 0).unwrap(), chrono::Utc)
            } else if let Ok(ts) = value.parse::<i64>() {
                if value.len() > 11 { // ms
                    chrono::DateTime::from_timestamp_millis(ts).unwrap_or_default()
                } else { // sec
                    chrono::DateTime::from_timestamp(ts, 0).unwrap_or_default()
                }
            } else {
                return Err(format!("Failed to parse date: {}", value));
            };

            match action {
                "format" | "formatDate" => {
                    let format = node.config.get("toFormat").and_then(|v| v.as_str()).or_else(|| node.config.get("format").and_then(|v| v.as_str())).unwrap_or("%Y-%m-%d %H:%M:%S");
                    let output_field = node.config.get("dataPropertyName").and_then(|v| v.as_str()).or_else(|| node.config.get("outputFieldName").and_then(|v| v.as_str())).unwrap_or("data");
                    
                    // Map common n8n formats to chrono formats
                    let rust_format = match format {
                        "YYYY-MM-DD" => "%Y-%m-%d",
                        "MM/DD/YYYY" => "%m/%d/%Y",
                        "YYYY/MM/DD" => "%Y/%m/%d",
                        _ => format
                    };
                    
                    Ok(serde_json::json!({ output_field: dt.format(rust_format).to_string() }))
                },
                "calculate" | "addToDate" | "subtractFromDate" => {
                    let operation = node.config.get("operation").and_then(|v| v.as_str()).unwrap_or("add");
                    let duration = node.config.get("duration").and_then(|v| v.as_i64()).unwrap_or(0);
                    let unit = node.config.get("timeUnit").and_then(|v| v.as_str()).unwrap_or("days");
                    let output_field = node.config.get("dataPropertyName").and_then(|v| v.as_str()).or_else(|| node.config.get("outputFieldName").and_then(|v| v.as_str())).unwrap_or("data");

                    let signed_duration = if operation == "subtract" || action == "subtractFromDate" { -duration } else { duration };
                    
                    let new_dt = match unit {
                        "seconds" => dt + chrono::Duration::seconds(signed_duration),
                        "minutes" => dt + chrono::Duration::minutes(signed_duration),
                        "hours" => dt + chrono::Duration::hours(signed_duration),
                        "days" => dt + chrono::Duration::days(signed_duration),
                        "weeks" => dt + chrono::Duration::weeks(signed_duration),
                        "months" => dt + chrono::Duration::days(signed_duration * 30), // Approx
                        "years" => dt + chrono::Duration::days(signed_duration * 365), // Approx
                        _ => dt + chrono::Duration::days(signed_duration),
                    };
                    
                    Ok(serde_json::json!({ output_field: new_dt.to_rfc3339() }))
                },
                "extractDate" => {
                    let part = node.config.get("part").and_then(|v| v.as_str()).unwrap_or("month");
                    let output_field = node.config.get("outputFieldName").and_then(|v| v.as_str()).unwrap_or("datePart");
                    
                    use chrono::Datelike;
                    use chrono::Timelike;
                    
                    let val = match part {
                        "year" => dt.year() as i64,
                        "month" => dt.month() as i64,
                        "day" => dt.day() as i64,
                        "hour" => dt.hour() as i64,
                        "minute" => dt.minute() as i64,
                        "second" => dt.second() as i64,
                        _ => 0
                    };
                    
                    Ok(serde_json::json!({ output_field: val }))
                }
                _ => Err(format!("Unsupported action: {}", action))
            }
        }
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

    // 1. Get connected tool nodes via the "tools" port
    let tool_nodes: Vec<&Node> = edges.iter()
        .filter(|e| e.to == node.id && e.to_port == Some("tools".to_string()))
        .filter_map(|e| all_nodes.iter().find(|n| n.id == e.from))
        .filter(|n| n.kind == "tool")
        .collect();

    let mut tools_schema = Vec::new();
    let mut mcp_tools_map = std::collections::HashMap::new();

    for tool_node in &tool_nodes {
        // 1.1 Check if it's an MCP tool
        if let Some(mcp_server_id_raw) = tool_node.config.get("mcpServerId").and_then(|v| v.as_str()) {
            if let Ok(mcp_id) = Uuid::parse_str(mcp_server_id_raw) {
                // Fetch specific MCP server
                let server = sqlx::query_as::<_, McpServer>("SELECT * FROM mcp_servers WHERE id = $1")
                    .bind(mcp_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?
                    .ok_or(format!("MCP Server not found: {}", mcp_server_id_raw))?;

                let target_tool_name = tool_node.config.get("toolName").and_then(|v| v.as_str()).ok_or("Tool name not specified in tool node")?;
                
                // Fetch tools from this server to get the schema
                let tools = fetch_mcp_tools(&server).await?;
                if let Some(tool) = tools.into_iter().find(|t| t.name == target_tool_name) {
                    let full_name = format!("{}__{}", server.name, tool.name);
                    tools_schema.push(serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": full_name,
                            "description": tool.description.unwrap_or_default(),
                            "parameters": tool.input_schema
                        }
                    }));
                    mcp_tools_map.insert(full_name, (server.clone(), tool.name.clone()));
                } else {
                    return Err(format!("Tool '{}' not found on MCP server '{}'", target_tool_name, server.name));
                }
            }
        } 
        // 1.2 Check if it's an RSS Read Tool
        else if tool_node.kind == "rss-read-tool" {
            let tool_name = tool_node.config.get("toolName").and_then(|v| v.as_str()).unwrap_or("rss_reader");
            let description = tool_node.config.get("description").and_then(|v| v.as_str()).unwrap_or("Reads entries from an RSS feed.");
            tools_schema.push(serde_json::json!({
                "type": "function",
                "function": {
                    "name": tool_name,
                    "description": description,
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "query": { "type": "string", "description": "Optional search query to filter feed items" }
                        }
                    }
                }
            }));
        }
        else {
            // Static/Manual Tool Node (if any)
            let tool_name = tool_node.config.get("toolName").and_then(|v| v.as_str()).unwrap_or("unknown_tool");
            let description = tool_node.config.get("description").and_then(|v| v.as_str()).unwrap_or("No description");
            tools_schema.push(serde_json::json!({
                "type": "function",
                "function": {
                    "name": tool_name,
                    "description": description,
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "query": { "type": "string" }
                        }
                    }
                }
            }));
        }
    }

    let api_key = if provider == "openai" {
        get_api_key(pool, node, "openai", "OPENAI_API_KEY").await?
    } else {
        get_api_key(pool, node, "openrouter", "OPENROUTER_API_KEY").await?
    };

    let mut current_messages = Vec::new();
    if let Some(s) = system_message {
        current_messages.push(OpenAiMessage { role: "system".to_string(), content: s, tool_calls: None, tool_call_id: None });
    }
    current_messages.push(OpenAiMessage { role: "user".to_string(), content: prompt, tool_calls: None, tool_call_id: None });

    let tools_value = if !tools_schema.is_empty() {
        Some(serde_json::Value::Array(tools_schema))
    } else {
        None
    };

    for _ in 0..10 {
        let response = if provider == "openai" {
            let client = OpenAiClient::new(api_key.clone());
            client.generate(model, current_messages.clone(), None, None, tools_value.clone()).await.map_err(|e| e.to_string())?
        } else {
            let client = OpenRouterClient::new(api_key.clone());
            let or_messages: Vec<OpenRouterMessage> = current_messages.iter().map(|m| OpenRouterMessage {
                role: m.role.clone(),
                content: m.content.clone(),
                tool_calls: m.tool_calls.clone(),
                tool_call_id: m.tool_call_id.clone()
            }).collect();
            let or_request = OpenRouterRequest {
                model: model.to_string(),
                messages: or_messages,
                temperature: None,
                max_tokens: None,
                top_p: None,
                frequency_penalty: None,
                presence_penalty: None,
                response_format: None,
                tools: tools_value.clone(),
                tool_choice: None
            };
            client.generate(or_request).await.map_err(|e| e.to_string())?
        };

        let message = response.get("choices")
            .and_then(|c| c.get(0))
            .and_then(|m| m.get("message"))
            .ok_or("Invalid LLM response")?;

        if let Some(tool_calls) = message.get("tool_calls") {
            current_messages.push(OpenAiMessage {
                role: "assistant".to_string(),
                content: message.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                tool_calls: Some(tool_calls.clone()),
                tool_call_id: None
            });

            for call in tool_calls.as_array().unwrap_or(&vec![]) {
                let call_id = call.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let func_name = call.get("function").and_then(|f| f.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                let arguments = call.get("function").and_then(|f| f.get("arguments")).and_then(|v| v.as_str()).unwrap_or("{}");
                let args_json: serde_json::Value = serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));

                let tool_result = if let Some((server, original_name)) = mcp_tools_map.get(func_name) {
                    match call_mcp_tool(server, original_name, args_json).await {
                        Ok(res) => res.to_string(),
                        Err(e) => format!("Error calling MCP tool: {}", e),
                    }
                } else {
                    let tool_node = tool_nodes.iter().find(|tn| tn.config.get("toolName").and_then(|v| v.as_str()) == Some(func_name));
                    
                    if let Some(tn) = tool_node {
                        if tn.kind == "rss-read-tool" {
                            // Execute RSS Read logic for Agent (Inlined to avoid async recursion)
                            let url_raw = tn.config.get("url").and_then(|v| v.as_str()).unwrap_or("");
                            if url_raw.is_empty() {
                                "Error: RSS Feed URL not configured in tool node".to_string()
                            } else {
                                let url = interpolate_value(url_raw, input);
                                let client = reqwest::Client::new();
                                match client.get(&url).send().await {
                                    Ok(resp) => {
                                        match resp.bytes().await {
                                            Ok(content) => {
                                                match feed_rs::parser::parse(&content[..]) {
                                                    Ok(feed) => {
                                                        let mut items = Vec::new();
                                                        for entry in feed.entries {
                                                            items.push(serde_json::json!({
                                                                "title": entry.title.map(|t| t.content),
                                                                "link": entry.links.first().map(|l| l.href.clone()),
                                                                "published": entry.published,
                                                            }));
                                                        }
                                                        serde_json::json!(items).to_string()
                                                    },
                                                    Err(e) => format!("Error parsing RSS: {}", e)
                                                }
                                            },
                                            Err(e) => format!("Error reading bytes: {}", e)
                                        }
                                    },
                                    Err(e) => format!("Error fetching feed: {}", e)
                                }
                            }
                        } else {
                            format!("Result from {}: Action completed successfully.", func_name)
                        }
                    } else {
                        format!("Error: Tool '{}' not found", func_name)
                    }
                };

                current_messages.push(OpenAiMessage {
                    role: "tool".to_string(),
                    content: tool_result,
                    tool_calls: None,
                    tool_call_id: Some(call_id.to_string())
                });
            }
        } else {
            let text = message.get("content").and_then(|v| v.as_str()).unwrap_or("");
            return Ok(serde_json::json!({ "text": text }));
        }
    }

    Err("Agent reached maximum iterations".to_string())
}

async fn fetch_mcp_tools(server: &McpServer) -> Result<Vec<rmcp::model::Tool>, String> {
    if server.transport != "streamable-http" { return Ok(vec![]); }
    let Some(url) = &server.endpoint else { return Ok(vec![]); };

    use rmcp::ServiceExt;
    use rmcp::transport::StreamableHttpClientTransport;
    use rmcp::transport::streamable_http_client::StreamableHttpClientTransportConfig;
    use rmcp::model::{ClientCapabilities, ClientInfo, Implementation};

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
        .map_err(|e| e.to_string())?;

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

    let client = client_info.serve(transport).await.map_err(|e| e.to_string())?;
    let tools = client.peer().list_all_tools().await.map_err(|e| e.to_string())?;
    let _ = client.cancel().await;

    Ok(tools)
}

async fn call_mcp_tool(server: &McpServer, tool_name: &str, arguments: serde_json::Value) -> Result<serde_json::Value, String> {
    if server.transport != "streamable-http" { return Err("Unsupported transport".to_string()); }
    let Some(url) = &server.endpoint else { return Err("Missing endpoint".to_string()); };

    use rmcp::ServiceExt;
    use rmcp::transport::StreamableHttpClientTransport;
    use rmcp::transport::streamable_http_client::StreamableHttpClientTransportConfig;
    use rmcp::model::{ClientCapabilities, ClientInfo, Implementation, CallToolRequestParams};

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
        .map_err(|e| e.to_string())?;

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

    let client = client_info.serve(transport).await.map_err(|e| e.to_string())?;
    
    let args_map = arguments.as_object().cloned().unwrap_or_default();
    
    let params = CallToolRequestParams {
        name: tool_name.to_string().into(),
        arguments: Some(args_map),
        meta: None,
        task: None,
    };
    
    let result = client.peer().call_tool(params).await.map_err(|e| e.to_string())?;


    Ok(serde_json::to_value(result.content).unwrap_or(serde_json::json!([])))
}