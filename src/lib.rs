//! TOON - A compact, lossless JSON encoding format
//! 
//! TOON provides two encoding modes:
//! - Text mode: Indentation-based, human-readable format
//! - Compact mode: Binary length-prefixed format for maximum compression

pub mod codec;
pub mod decoder;
pub mod encoder;

use serde_json::Value;

/// Options for encoding JSON to TOON
#[derive(Default, Clone, Debug)]
pub struct EncodeOptions {
    /// Use tabular layout for uniform arrays of objects
    pub tabular_arrays: bool,
    /// Use compact binary format
    pub compact: bool,
    /// Indentation in spaces (for text mode)
    pub indent: Option<u8>,
    /// Fail on validation errors
    pub strict: bool,
}

/// Options for decoding TOON to JSON
#[derive(Default, Clone, Debug)]
pub struct DecodeOptions {
    /// Expect compact format (auto-detect if false)
    pub compact: bool,
    /// Fail on validation errors
    pub strict: bool,
}

/// Encode a JSON value to TOON format
pub fn encode_json_to_toon(input: &Value, opt: &EncodeOptions) -> anyhow::Result<Vec<u8>> {
    encoder::encode(input, opt)
}

/// Decode TOON bytes to a JSON value
pub fn decode_toon_to_json(bytes: &[u8], opt: &DecodeOptions) -> anyhow::Result<Value> {
    decoder::decode(bytes, opt)
}
