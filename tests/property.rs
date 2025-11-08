use proptest::prelude::*;
use serde_json::Value;
use toon::{decode_toon_to_json, encode_json_to_toon, DecodeOptions, EncodeOptions};

fn json_value_strategy() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<i32>().prop_map(|n| Value::Number(n.into())),
        // Use integers as floats to avoid precision issues in roundtrip
        (-10000..10000_i32).prop_map(|n| {
            serde_json::Number::from_f64(n as f64)
            .map(Value::Number)
            .unwrap_or(Value::Null)
        }),
        "[a-zA-Z_][a-zA-Z0-9_]{0,19}".prop_map(Value::String),
    ];

    leaf.prop_recursive(
        3,   // max depth
        256, // max nodes
        10,  // items per collection
        |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..10).prop_map(Value::Array),
                        prop::collection::hash_map("[a-z]{1,10}", inner, 0..10).prop_map(|m| {
                            Value::Object(m.into_iter().collect())
                        }),
            ]
        },
    )
}

proptest! {
    #[test]
    fn test_roundtrip_text_mode(value in json_value_strategy()) {
        let opts = EncodeOptions::default();
        let encoded = encode_json_to_toon(&value, &opts)
        .map_err(|e| TestCaseError::fail(e.to_string()))?;
        let decoded = decode_toon_to_json(&encoded, &DecodeOptions::default())
        .map_err(|e| TestCaseError::fail(e.to_string()))?;
        prop_assert_eq!(value, decoded);
    }

    #[test]
    fn test_roundtrip_compact_mode(value in json_value_strategy()) {
        let opts = EncodeOptions {
            compact: true,
            ..Default::default()
        };
        let encoded = encode_json_to_toon(&value, &opts)
        .map_err(|e| TestCaseError::fail(e.to_string()))?;
        let decode_opts = DecodeOptions {
            compact: true,
            ..Default::default()
        };
        let decoded = decode_toon_to_json(&encoded, &decode_opts)
        .map_err(|e| TestCaseError::fail(e.to_string()))?;
        prop_assert_eq!(value, decoded);
    }
}
