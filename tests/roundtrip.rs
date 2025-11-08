use serde_json::{json, Value};
use toon::{decode_toon_to_json, encode_json_to_toon, DecodeOptions, EncodeOptions};

fn roundtrip_test(value: &Value, opts: &EncodeOptions) {
    let encoded = encode_json_to_toon(value, opts).expect("Encode failed");
    let decode_opts = DecodeOptions {
        compact: opts.compact,
        strict: opts.strict,
    };
    let decoded = decode_toon_to_json(&encoded, &decode_opts).expect("Decode failed");
    assert_eq!(*value, decoded, "Roundtrip failed");
}

#[test]
fn test_null() {
    let value = json!(null);
    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_booleans() {
    roundtrip_test(&json!(true), &EncodeOptions::default());
    roundtrip_test(&json!(false), &EncodeOptions::default());
    roundtrip_test(&json!(true), &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_numbers() {
    let values = vec![
        json!(0),
        json!(42),
        json!(-42),
        json!(3.14159),
        json!(-3.14159),
        json!(1e10),
        json!(1.5e-8),
    ];

    for val in values {
        roundtrip_test(&val, &EncodeOptions::default());
        roundtrip_test(&val, &EncodeOptions {
            compact: true,
            ..Default::default()
        });
    }
}

#[test]
fn test_strings() {
    let values = vec![
        json!(""),
        json!("hello"),
        json!("hello world"),
        json!("with\nnewlines"),
        json!("with\ttabs"),
        json!("with\"quotes\""),
        json!("unicode: 日本語"),
        json!("emoji: 🎉"),
    ];

    for val in values {
        roundtrip_test(&val, &EncodeOptions::default());
        roundtrip_test(&val, &EncodeOptions {
            compact: true,
            ..Default::default()
        });
    }
}

#[test]
fn test_empty_collections() {
    roundtrip_test(&json!([]), &EncodeOptions::default());
    roundtrip_test(&json!({}), &EncodeOptions::default());
    roundtrip_test(&json!([]), &EncodeOptions {
        compact: true,
        ..Default::default()
    });
    roundtrip_test(&json!({}), &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_arrays_of_primitives() {
    let values = vec![
        json!([1, 2, 3]),
        json!(["a", "b", "c"]),
        json!([true, false, true]),
        json!([null, null, null]),
        json!([1, "two", true, null]),
    ];

    for val in values {
        roundtrip_test(&val, &EncodeOptions::default());
        roundtrip_test(&val, &EncodeOptions {
            compact: true,
            ..Default::default()
        });
    }
}

#[test]
fn test_flat_object() {
    let value = json!({
        "name": "Alice",
        "age": 30,
        "active": true,
        "score": null
    });

    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_nested_objects() {
    let value = json!({
        "user": {
            "name": "Bob",
            "address": {
                "street": "123 Main St",
                "city": "Springfield"
            }
        },
        "metadata": {
            "created": "2024-01-01",
            "tags": ["important", "urgent"]
        }
    });

    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_array_of_objects_uniform() {
    let value = json!([
        {"id": 1, "name": "Alice", "score": 100},
        {"id": 2, "name": "Bob", "score": 95},
        {"id": 3, "name": "Charlie", "score": 88}
    ]);

    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_array_of_objects_nonuniform() {
    let value = json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob", "extra": "field"},
        {"name": "Charlie", "score": 88}
    ]);

    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_nested_arrays() {
    let value = json!([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]);

    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_mixed_nesting() {
    let value = json!({
        "data": [
            {"items": [1, 2, 3]},
            {"items": [4, 5]},
            {"items": []}
        ],
        "metadata": {
            "count": 3,
            "nested": {
                "deep": {
                    "value": "test"
                }
            }
        }
    });

    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}

#[test]
fn test_array_order_preserved() {
    let value = json!(["z", "a", "m", "b"]);
    let encoded = encode_json_to_toon(&value, &EncodeOptions::default()).unwrap();
    let decoded = decode_toon_to_json(&encoded, &DecodeOptions::default()).unwrap();
    
    assert_eq!(value, decoded);
    if let Value::Array(arr) = decoded {
        assert_eq!(arr[0], "z");
        assert_eq!(arr[1], "a");
        assert_eq!(arr[2], "m");
        assert_eq!(arr[3], "b");
    } else {
        panic!("Expected array");
    }
}

#[test]
fn test_large_document() {
    let mut items = Vec::new();
    for i in 0..100 {
        items.push(json!({
            "id": i,
            "name": format!("Item {}", i),
            "value": i as f64 * 1.5,
            "tags": ["a", "b", "c"],
            "meta": {
                "created": "2024-01-01",
                "updated": "2024-01-02"
            }
        }));
    }
    let value = json!(items);

    roundtrip_test(&value, &EncodeOptions::default());
    roundtrip_test(&value, &EncodeOptions {
        compact: true,
        ..Default::default()
    });
}
