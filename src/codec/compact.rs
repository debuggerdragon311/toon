use anyhow::{Context, Result};
use serde_json::Value;

const MAGIC: &[u8] = b"TOON\x01";

// Type tags
const TAG_NULL: u8 = 0;
const TAG_FALSE: u8 = 1;
const TAG_TRUE: u8 = 2;
const TAG_NUMBER: u8 = 3;
const TAG_STRING: u8 = 4;
const TAG_ARRAY: u8 = 5;
const TAG_OBJECT: u8 = 6;

pub fn encode(value: &Value) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    buf.extend_from_slice(MAGIC);
    encode_value(&mut buf, value)?;
    Ok(buf)
}

fn encode_value(buf: &mut Vec<u8>, value: &Value) -> Result<()> {
    match value {
        Value::Null => buf.push(TAG_NULL),
        Value::Bool(false) => buf.push(TAG_FALSE),
        Value::Bool(true) => buf.push(TAG_TRUE),
        Value::Number(n) => {
            buf.push(TAG_NUMBER);
            let s = n.to_string();
            write_string(buf, &s);
        }
        Value::String(s) => {
            buf.push(TAG_STRING);
            write_string(buf, s);
        }
        Value::Array(arr) => {
            buf.push(TAG_ARRAY);
            write_u32(buf, arr.len() as u32);
            for item in arr {
                encode_value(buf, item)?;
            }
        }
        Value::Object(obj) => {
            buf.push(TAG_OBJECT);
            write_u32(buf, obj.len() as u32);
            
            // Sort keys for deterministic output
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            
            for key in keys {
                write_string(buf, key);
                encode_value(buf, &obj[key])?;
            }
        }
    }
    Ok(())
}

fn write_u32(buf: &mut Vec<u8>, val: u32) {
    buf.extend_from_slice(&val.to_le_bytes());
}

fn write_string(buf: &mut Vec<u8>, s: &str) {
    write_u32(buf, s.len() as u32);
    buf.extend_from_slice(s.as_bytes());
}

pub fn decode(bytes: &[u8]) -> Result<Value> {
    if bytes.len() < MAGIC.len() {
        anyhow::bail!("Input too short for compact TOON");
    }
    if &bytes[..MAGIC.len()] != MAGIC {
        anyhow::bail!("Invalid compact TOON magic header");
    }

    let mut pos = MAGIC.len();
    decode_value(bytes, &mut pos)
}

fn decode_value(bytes: &[u8], pos: &mut usize) -> Result<Value> {
    if *pos >= bytes.len() {
        anyhow::bail!("Unexpected end of input");
    }

    let tag = bytes[*pos];
    *pos += 1;

    match tag {
        TAG_NULL => Ok(Value::Null),
        TAG_FALSE => Ok(Value::Bool(false)),
        TAG_TRUE => Ok(Value::Bool(true)),
        TAG_NUMBER => {
            let s = read_string(bytes, pos)?;
            let n: serde_json::Number = s
                .parse()
                .with_context(|| format!("Invalid number in compact TOON: {}", s))?;
            Ok(Value::Number(n))
        }
        TAG_STRING => {
            let s = read_string(bytes, pos)?;
            Ok(Value::String(s))
        }
        TAG_ARRAY => {
            let len = read_u32(bytes, pos)? as usize;
            let mut arr = Vec::with_capacity(len);
            for _ in 0..len {
                arr.push(decode_value(bytes, pos)?);
            }
            Ok(Value::Array(arr))
        }
        TAG_OBJECT => {
            let len = read_u32(bytes, pos)? as usize;
            let mut obj = serde_json::Map::new();
            for _ in 0..len {
                let key = read_string(bytes, pos)?;
                let value = decode_value(bytes, pos)?;
                obj.insert(key, value);
            }
            Ok(Value::Object(obj))
        }
        _ => anyhow::bail!("Unknown type tag: {}", tag),
    }
}

fn read_u32(bytes: &[u8], pos: &mut usize) -> Result<u32> {
    if *pos + 4 > bytes.len() {
        anyhow::bail!("Unexpected end of input reading u32");
    }
    let val = u32::from_le_bytes([
        bytes[*pos],
        bytes[*pos + 1],
        bytes[*pos + 2],
        bytes[*pos + 3],
    ]);
    *pos += 4;
    Ok(val)
}

fn read_string(bytes: &[u8], pos: &mut usize) -> Result<String> {
    let len = read_u32(bytes, pos)? as usize;
    if *pos + len > bytes.len() {
        anyhow::bail!("Unexpected end of input reading string");
    }
    let s = std::str::from_utf8(&bytes[*pos..*pos + len])
        .context("Invalid UTF-8 in string")?;
    *pos += len;
    Ok(s.to_string())
}
