use anyhow::Result;
use serde_json::Value;
use std::collections::HashSet;

const TABULAR_MAGIC: &[u8] = b"TOON-TAB\x01";

pub fn is_uniform_object_array(arr: &[Value]) -> bool {
    if arr.is_empty() {
        return false;
    }

    // Check first element is an object
    let first_keys = match &arr[0] {
        Value::Object(obj) => {
            let keys: HashSet<_> = obj.keys().collect();
            if keys.is_empty() {
                return false;
            }
            keys
        }
        _ => return false,
    };

    // Check all elements have same keys
    for item in &arr[1..] {
        match item {
            Value::Object(obj) => {
                let keys: HashSet<_> = obj.keys().collect();
                if keys != first_keys {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

pub fn encode_tabular_text(arr: &[Value], indent: u8) -> Result<Vec<u8>> {
    if arr.is_empty() {
        return Ok(b"[]".to_vec());
    }

    let keys = extract_keys(&arr[0])?;
    let mut output = String::new();

    // Header
    output.push_str("[\n");
    let indent_str = " ".repeat(indent as usize);
    output.push_str(&indent_str);
    output.push_str("# ");
    for (i, key) in keys.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(key);
    }
    output.push('\n');

    // Rows
    for (row_idx, item) in arr.iter().enumerate() {
        output.push_str(&indent_str);
        if let Value::Object(obj) = item {
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                if let Some(val) = obj.get(key) {
                    append_value_inline(&mut output, val)?;
                } else {
                    output.push_str("null");
                }
            }
        }
        if row_idx < arr.len() - 1 {
            output.push(',');
        }
        output.push('\n');
    }

    output.push(']');
    Ok(output.into_bytes())
}

pub fn encode_tabular_compact(arr: &[Value]) -> Result<Vec<u8>> {
    if arr.is_empty() {
        return Ok(b"[]".to_vec());
    }

    let keys = extract_keys(&arr[0])?;
    let mut buf = Vec::new();

    buf.extend_from_slice(TABULAR_MAGIC);

    // Write key count and keys
    write_u32(&mut buf, keys.len() as u32);
    for key in &keys {
        write_string(&mut buf, key);
    }

    // Write row count
    write_u32(&mut buf, arr.len() as u32);

    // Write rows
    for item in arr {
        if let Value::Object(obj) = item {
            for key in &keys {
                if let Some(val) = obj.get(key) {
                    encode_compact_value(&mut buf, val)?;
                } else {
                    buf.push(0); // TAG_NULL
                }
            }
        } else {
            anyhow::bail!("Non-object in tabular array");
        }
    }

    Ok(buf)
}

fn extract_keys(value: &Value) -> Result<Vec<String>> {
    match value {
        Value::Object(obj) => {
            let mut keys: Vec<_> = obj.keys().cloned().collect();
            keys.sort();
            Ok(keys)
        }
        _ => anyhow::bail!("Expected object for tabular encoding"),
    }
}

fn append_value_inline(out: &mut String, val: &Value) -> Result<()> {
    match val {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&n.to_string()),
        Value::String(s) => {
            if needs_quotes(s) {
                out.push('"');
                for ch in s.chars() {
                    match ch {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        '\r' => out.push_str("\\r"),
                        '\t' => out.push_str("\\t"),
                        c => out.push(c),
                    }
                }
                out.push('"');
            } else {
                out.push_str(s);
            }
        }
        Value::Array(_) | Value::Object(_) => {
            // Nested structures as JSON
            out.push_str(&serde_json::to_string(val)?);
        }
    }
    Ok(())
}

fn needs_quotes(s: &str) -> bool {
    s.is_empty()
        || s.chars()
            .any(|c| c.is_whitespace() || c == '"' || c == ',' || c == '[' || c == ']')
}

// Compact encoding helpers
fn write_u32(buf: &mut Vec<u8>, val: u32) {
    buf.extend_from_slice(&val.to_le_bytes());
}

fn write_string(buf: &mut Vec<u8>, s: &str) {
    write_u32(buf, s.len() as u32);
    buf.extend_from_slice(s.as_bytes());
}

fn encode_compact_value(buf: &mut Vec<u8>, value: &Value) -> Result<()> {
    match value {
        Value::Null => buf.push(0),
        Value::Bool(false) => buf.push(1),
        Value::Bool(true) => buf.push(2),
        Value::Number(n) => {
            buf.push(3);
            write_string(buf, &n.to_string());
        }
        Value::String(s) => {
            buf.push(4);
            write_string(buf, s);
        }
        Value::Array(_) | Value::Object(_) => {
            // Nested structures as JSON string
            buf.push(4);
            let json = serde_json::to_string(value)?;
            write_string(buf, &json);
        }
    }
    Ok(())
}
