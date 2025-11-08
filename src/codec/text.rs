use anyhow::{Context, Result};
use serde_json::Value;
use std::fmt::Write as FmtWrite;

pub fn encode(value: &Value, indent_size: u8) -> Result<Vec<u8>> {
    let mut output = String::new();
    encode_value(&mut output, value, 0, indent_size)?;
    Ok(output.into_bytes())
}

fn encode_value(out: &mut String, value: &Value, depth: usize, indent: u8) -> Result<()> {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => write!(out, "{}", n).unwrap(),
        Value::String(s) => encode_string(out, s)?,
        Value::Array(arr) => encode_array(out, arr, depth, indent)?,
        Value::Object(obj) => encode_object(out, obj, depth, indent)?,
    }
    Ok(())
}

fn encode_string(out: &mut String, s: &str) -> Result<()> {
    // Quote strings that need it
    let is_keyword = matches!(s, "true" | "false" | "null");

    // Check if string could be confused with a number
    let looks_like_number = !s.is_empty() && {
        let first = s.chars().next().unwrap();
        (first.is_ascii_digit() || first == '-' || first == '+') &&
        s.chars().all(|c| c.is_ascii_digit() || matches!(c, '-' | '+' | '.' | 'e' | 'E'))
    };

    let needs_quote = s.is_empty()
    || looks_like_number
    || is_keyword
    || s.chars().any(|c| {
        c.is_whitespace() || c == '"' || c == ':' || c == ',' || c == '{' || c == '}'
    || c == '[' || c == ']'
    });

    if needs_quote {
        out.push('"');
        for ch in s.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if c.is_control() => write!(out, "\\u{:04x}", c as u32).unwrap(),
                c => out.push(c),
            }
        }
        out.push('"');
    } else {
        out.push_str(s);
    }
    Ok(())
}

fn encode_array(out: &mut String, arr: &[Value], depth: usize, indent: u8) -> Result<()> {
    if arr.is_empty() {
        out.push_str("[]");
        return Ok(());
    }

    out.push('[');
    let indent_str = " ".repeat((depth + 1) * indent as usize);

    for (i, item) in arr.iter().enumerate() {
        out.push('\n');
        out.push_str(&indent_str);
        encode_value(out, item, depth + 1, indent)?;
        if i < arr.len() - 1 {
            out.push(',');
        }
    }

    out.push('\n');
    out.push_str(&" ".repeat(depth * indent as usize));
    out.push(']');
    Ok(())
}

fn encode_object(
    out: &mut String,
    obj: &serde_json::Map<String, Value>,
    depth: usize,
    indent: u8,
) -> Result<()> {
    if obj.is_empty() {
        out.push_str("{}");
        return Ok(());
    }

    out.push('{');
    let indent_str = " ".repeat((depth + 1) * indent as usize);

    let mut keys: Vec<_> = obj.keys().collect();
    keys.sort(); // Deterministic output

    for (i, key) in keys.iter().enumerate() {
        let value = &obj[*key];
        out.push('\n');
        out.push_str(&indent_str);
        encode_string(out, key)?;
        out.push_str(": ");
        encode_value(out, value, depth + 1, indent)?;
        if i < keys.len() - 1 {
            out.push(',');
        }
    }

    out.push('\n');
    out.push_str(&" ".repeat(depth * indent as usize));
    out.push('}');
    Ok(())
}

pub fn decode(bytes: &[u8]) -> Result<Value> {
    let s = std::str::from_utf8(bytes).context("Invalid UTF-8 in TOON text")?;
    parse_value(s.trim()).map(|(v, _)| v)
}

fn parse_value(s: &str) -> Result<(Value, &str)> {
    let s = s.trim_start();
    if s.is_empty() {
        anyhow::bail!("Unexpected end of input");
    }

    match s.chars().next().unwrap() {
        '{' => parse_object(s),
        '[' => parse_array(s),
        '"' => parse_quoted_string(s),
        't' if s.starts_with("true") => Ok((Value::Bool(true), &s[4..])),
        'f' if s.starts_with("false") => Ok((Value::Bool(false), &s[5..])),
        'n' if s.starts_with("null") => Ok((Value::Null, &s[4..])),
        '-' | '0'..='9' => {
            // Try number first, fall back to unquoted string
            match parse_number(s) {
                Ok(result) => Ok(result),
                Err(_) => parse_unquoted_string(s),
            }
        }
        _ => parse_unquoted_string(s),
    }
}

fn parse_object(s: &str) -> Result<(Value, &str)> {
    let mut s = &s[1..]; // skip '{'
    let mut obj = serde_json::Map::new();

    loop {
        s = s.trim_start();
        if s.starts_with('}') {
            return Ok((Value::Object(obj), &s[1..]));
        }

        // Parse key
        let (key, rest) = parse_key(s)?;
        s = rest.trim_start();

        if !s.starts_with(':') {
            anyhow::bail!("Expected ':' after object key");
        }
        s = &s[1..];

        // Parse value
        let (value, rest) = parse_value(s)?;
        obj.insert(key, value);
        s = rest.trim_start();

        if s.starts_with(',') {
            s = &s[1..];
        } else if !s.starts_with('}') {
            anyhow::bail!("Expected ',' or '}}' in object");
        }
    }
}

fn parse_array(s: &str) -> Result<(Value, &str)> {
    let mut s = &s[1..]; // skip '['
    let mut arr = Vec::new();

    loop {
        s = s.trim_start();
        if s.starts_with(']') {
            return Ok((Value::Array(arr), &s[1..]));
        }

        let (value, rest) = parse_value(s)?;
        arr.push(value);
        s = rest.trim_start();

        if s.starts_with(',') {
            s = &s[1..];
        } else if !s.starts_with(']') {
            anyhow::bail!("Expected ',' or ']' in array");
        }
    }
}

fn parse_key(s: &str) -> Result<(String, &str)> {
    let s = s.trim_start();
    if s.starts_with('"') {
        let (val, rest) = parse_quoted_string(s)?;
        match val {
            Value::String(k) => Ok((k, rest)),
            _ => unreachable!(),
        }
    } else {
        parse_unquoted_key(s)
    }
}

fn parse_quoted_string(s: &str) -> Result<(Value, &str)> {
    let mut chars = s[1..].chars();
    let mut result = String::new();
    let mut escaped = false;

    loop {
        match chars.next() {
            None => anyhow::bail!("Unterminated string"),
            Some('"') if !escaped => {
                let consumed = s.len() - chars.as_str().len();
                return Ok((Value::String(result), &s[consumed..]));
            }
            Some('\\') if !escaped => {
                escaped = true;
            }
            Some(c) if escaped => {
                escaped = false;
                match c {
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'u' => {
                        let hex: String = chars.by_ref().take(4).collect();
                        let code = u32::from_str_radix(&hex, 16)
                        .context("Invalid unicode escape")?;
                        result.push(
                            char::from_u32(code)
                            .ok_or_else(|| anyhow::anyhow!("Invalid unicode codepoint"))?,
                        );
                    }
                    _ => anyhow::bail!("Invalid escape sequence: \\{}", c),
                }
            }
            Some(c) => {
                result.push(c);
            }
        }
    }
}

fn parse_unquoted_string(s: &str) -> Result<(Value, &str)> {
    let end = s
    .find(|c: char| c.is_whitespace() || c == ',' || c == '}' || c == ']' || c == ':')
    .unwrap_or(s.len());

    if end == 0 {
        anyhow::bail!("Expected value");
    }

    Ok((Value::String(s[..end].to_string()), &s[end..]))
}

fn parse_unquoted_key(s: &str) -> Result<(String, &str)> {
    let end = s
    .find(|c: char| c.is_whitespace() || c == ':')
    .unwrap_or(s.len());

    if end == 0 {
        anyhow::bail!("Expected key");
    }

    Ok((s[..end].to_string(), &s[end..]))
}

fn parse_number(s: &str) -> Result<(Value, &str)> {
    let end = s
    .find(|c: char| {
        !matches!(c, '0'..='9' | '-' | '+' | '.' | 'e' | 'E')
    })
    .unwrap_or(s.len());

    if end == 0 {
        anyhow::bail!("Expected number");
    }

    let num_str = &s[..end];
    let num: serde_json::Number = num_str
    .parse()
    .with_context(|| format!("Invalid number: {}", num_str))?;
    Ok((Value::Number(num), &s[end..]))
}
