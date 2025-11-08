use crate::codec::{compact, text};
use crate::DecodeOptions;
use anyhow::{Context, Result};
use serde_json::Value;

const COMPACT_MAGIC: &[u8] = b"TOON\x01";

pub fn decode(bytes: &[u8], opt: &DecodeOptions) -> Result<Value> {
    if bytes.is_empty() {
        anyhow::bail!("Empty input");
    }

    // Auto-detect format if not specified
    let is_compact = if opt.compact {
        true
    } else {
        bytes.len() >= COMPACT_MAGIC.len() && &bytes[..COMPACT_MAGIC.len()] == COMPACT_MAGIC
    };

    if is_compact {
        compact::decode(bytes).context("Failed to decode compact TOON")
    } else {
        text::decode(bytes).context("Failed to decode text TOON")
    }
}
