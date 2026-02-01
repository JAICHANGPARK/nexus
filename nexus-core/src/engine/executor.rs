use rquickjs::{Context, Runtime};
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose};

pub struct CodeExecutor;

impl CodeExecutor {
    pub async fn execute_js(code: &str, input: &Value) -> Result<Value, String> {
        let runtime = Runtime::new().map_err(|e| e.to_string())?;
        let context = Context::full(&runtime).map_err(|e| e.to_string())?;

        // Prepare input JSON
        let input_json = serde_json::to_string(input).unwrap_or_else(|_| "{}".to_string());
        
        context.with(|ctx| {
            let globals = ctx.globals();
            
            // Inject input data safely
            // We use JSON.parse on a stringified version to ensure proper object creation in JS
            let input_json_escaped = input_json.replace("\\", "\\\\").replace("'", "\\'");
            
            let init_script = format!(
                r#" 
                (function() {{ 
                    const rawInput = JSON.parse('{}');
                    const items = Array.isArray(rawInput) ? rawInput : [rawInput];
                    
                    // Map to n8n style [{{ json: {{ ... }} }}] if not already
                    const normalizedItems = items.map(item => {{
                        if (item && typeof item === 'object' && 'json' in item) return item;
                        return {{ json: item }};
                    }});

                    globalThis.$input = {{
                        all: () => normalizedItems,
                        first: () => normalizedItems[0],
                        last: () => normalizedItems[normalizedItems.length - 1]
                    }};
                }})();
                "#,
                input_json_escaped
            );
            
            ctx.eval::<(), _>(init_script).map_err(|e| format!("Init Error: {}", e))?;

            // Execute user code
            // Wrap in an immediately invoked function to support 'return' at top level
            let wrapped_code = format!("(async () => {{ \n{}\n }})()", code);
            
            match ctx.eval::<rquickjs::Value, _>(wrapped_code) {
                Ok(res) => {
                    let json_mod = globals.get::<_, rquickjs::Object>("JSON").map_err(|e| e.to_string())?;
                    let stringify = json_mod.get::<_, rquickjs::Function>("stringify").map_err(|e| e.to_string())?;
                    
                    let json_str: String = stringify.call((res,)).map_err(|e| format!("Serialization Error: {}", e))?;
                    let val: Value = serde_json::from_str(&json_str).unwrap_or(serde_json::json!({}));
                    Ok(val)
                }
                Err(e) => Err(format!("JS Execution Error: {}", e)),
            }
        })
    }

    pub async fn execute_python(code: &str, input: &Value) -> Result<Value, String> {
        use std::process::Stdio;
        use tokio::process::Command;

        let input_json = serde_json::to_string(input).unwrap_or_else(|_| "{}".to_string());
        let b64_input = general_purpose::STANDARD.encode(input_json);
        
        // Python doesn't allow '$' in identifiers. Using 'data' as the variable name.
        let wrapper = format!(r#" 
import json
import sys
import base64

try:
    input_str = base64.b64decode('{}').decode('utf-8')
    data = json.loads(input_str)
    
    def main(data):
        # User code starts here
{}

    result = main(data)
    print(json.dumps(result))
except Exception as e:
    print(json.dumps({{"error": str(e)}}), file=sys.stderr)
    sys.exit(1)
"#,
            b64_input,
            indent_code(code)
        );

        let child = Command::new("python3")
            .arg("-c")
            .arg(wrapper)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn python3: {}", e))?;

        let output = child.wait_with_output().await.map_err(|e| e.to_string())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let val: Value = serde_json::from_str(stdout.trim()).unwrap_or(serde_json::json!({ "output": stdout.trim() }));
            Ok(val)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Python Error: {}", stderr.trim()))
        }
    }
}

fn indent_code(code: &str) -> String {
    code.lines()
        .map(|line| format!("        {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}
