use serde_json::json;
use toon::{decode_toon_to_json, encode_json_to_toon, DecodeOptions, EncodeOptions};

#[test]
fn test_uniform_array_tabular_text() {
    let value = json!([
        {"id": 1, "name": "Alice", "score": 100},
        {"id": 2, "name": "Bob", "score": 95},
        {"id": 3, "name": "Charlie", "score": 88}
    ]);

    let opts = EncodeOptions {
        tabular_arrays: true,
        ..Default::default()
    };

    let encoded = encode_json_to_toon(&value, &opts).expect("Encode failed");
    let decoded = decode_toon_to_json(&encoded, &DecodeOptions::default()).expect("Decode failed");

    // Should roundtrip correctly
    assert_eq!(value, decoded);
}

#[test]
fn test_uniform_array_tabular_compact() {
    let value = json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"},
        {"id": 3, "name": "Charlie"}
    ]);

    let opts = EncodeOptions {
        tabular_arrays: true,
        compact: true,
        ..Default::default()
    };

    let encoded = encode_json_to_toon(&value, &opts).expect("Encode failed");
    let decoded = decode_toon_to_json(
        &encoded,
        &DecodeOptions {
            compact: true,
            ..Default::default()
        },
    )
    .expect("Decode failed");

    assert_eq!(value, decoded);
}

#[test]
fn test_nonuniform_array_fallback() {
    let value = json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob", "extra": "field"}
    ]);

    let opts = EncodeOptions {
        tabular_arrays: true,
        ..Default::default()
    };

    // Should fall back to normal encoding without error
    let encoded = encode_json_to_toon(&value, &opts).expect("Encode should succeed");
    let decoded = decode_toon_to_json(&encoded, &DecodeOptions::default()).expect("Decode failed");

    assert_eq!(value, decoded);
}

#[test]
fn test_nonuniform_array_strict_fails() {
    let value = json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob", "extra": "field"}
    ]);

    let opts = EncodeOptions {
        tabular_arrays: true,
        strict: true,
        ..Default::default()
    };

    // Should fail in strict mode
    let result = encode_json_to_toon(&value, &opts);
    assert!(result.is_err());
}

#[test]
fn test_empty_array_tabular() {
    let value = json!([]);

    let opts = EncodeOptions {
        tabular_arrays: true,
        ..Default::default()
    };

    let encoded = encode_json_to_toon(&value, &opts).expect("Encode failed");
    let decoded = decode_toon_to_json(&encoded, &DecodeOptions::default()).expect("Decode failed");

    assert_eq!(value, decoded);
}

#[test]
fn test_single_object_array_tabular() {
    let value = json!([
        {"id": 1, "name": "Alice", "score": 100}
    ]);

    let opts = EncodeOptions {
        tabular_arrays: true,
        ..Default::default()
    };

    let encoded = encode_json_to_toon(&value, &opts).expect("Encode failed");
    let decoded = decode_toon_to_json(&encoded, &DecodeOptions::default()).expect("Decode failed");

    assert_eq!(value, decoded);
}

#[test]
fn test_nested_values_in_tabular() {
    let value = json!([
        {"id": 1, "tags": ["a", "b"], "meta": {"x": 1}},
        {"id": 2, "tags": ["c"], "meta": {"x": 2}}
    ]);

    let opts = EncodeOptions {
        tabular_arrays: true,
        ..Default::default()
    };

    let encoded = encode_json_to_toon(&value, &opts).expect("Encode failed");
    let decoded = decode_toon_to_json(&encoded, &DecodeOptions::default()).expect("Decode failed");

    assert_eq!(value, decoded);
}
