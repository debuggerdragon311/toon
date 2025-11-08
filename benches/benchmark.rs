use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;
use toon::{decode_toon_to_json, encode_json_to_toon, DecodeOptions, EncodeOptions};

fn create_sample_data() -> serde_json::Value {
    let mut users = Vec::new();
    for i in 0..100 {
        users.push(json!({
            "id": i,
            "name": format!("User{}", i),
            "email": format!("user{}@example.com", i),
            "age": 20 + (i % 50),
            "active": i % 2 == 0,
            "tags": ["tag1", "tag2", "tag3"],
            "metadata": {
                "created": "2024-01-01",
                "updated": "2024-01-02",
                "score": i as f64 * 1.5
            }
        }));
    }
    json!({
        "users": users,
        "metadata": {
            "version": "1.0",
            "count": 100
        }
    })
}

fn bench_encode_text(c: &mut Criterion) {
    let data = create_sample_data();
    let opts = EncodeOptions::default();

    c.bench_function("encode_text", |b| {
        b.iter(|| encode_json_to_toon(black_box(&data), black_box(&opts)).unwrap())
    });
}

fn bench_decode_text(c: &mut Criterion) {
    let data = create_sample_data();
    let opts = EncodeOptions::default();
    let encoded = encode_json_to_toon(&data, &opts).unwrap();

    c.bench_function("decode_text", |b| {
        b.iter(|| {
            decode_toon_to_json(black_box(&encoded), black_box(&DecodeOptions::default())).unwrap()
        })
    });
}

fn bench_encode_compact(c: &mut Criterion) {
    let data = create_sample_data();
    let opts = EncodeOptions {
        compact: true,
        ..Default::default()
    };

    c.bench_function("encode_compact", |b| {
        b.iter(|| encode_json_to_toon(black_box(&data), black_box(&opts)).unwrap())
    });
}

fn bench_decode_compact(c: &mut Criterion) {
    let data = create_sample_data();
    let opts = EncodeOptions {
        compact: true,
        ..Default::default()
    };
    let encoded = encode_json_to_toon(&data, &opts).unwrap();

    c.bench_function("decode_compact", |b| {
        b.iter(|| {
            decode_toon_to_json(
                black_box(&encoded),
                black_box(&DecodeOptions {
                    compact: true,
                    ..Default::default()
                }),
            )
            .unwrap()
        })
    });
}

fn bench_encode_tabular(c: &mut Criterion) {
    let data = create_sample_data();
    let opts = EncodeOptions {
        tabular_arrays: true,
        ..Default::default()
    };

    c.bench_function("encode_tabular", |b| {
        b.iter(|| encode_json_to_toon(black_box(&data), black_box(&opts)).unwrap())
    });
}

fn bench_roundtrip_text(c: &mut Criterion) {
    let data = create_sample_data();
    let opts = EncodeOptions::default();

    c.bench_function("roundtrip_text", |b| {
        b.iter(|| {
            let encoded = encode_json_to_toon(black_box(&data), black_box(&opts)).unwrap();
            decode_toon_to_json(black_box(&encoded), black_box(&DecodeOptions::default())).unwrap()
        })
    });
}

fn bench_roundtrip_compact(c: &mut Criterion) {
    let data = create_sample_data();
    let opts = EncodeOptions {
        compact: true,
        ..Default::default()
    };

    c.bench_function("roundtrip_compact", |b| {
        b.iter(|| {
            let encoded = encode_json_to_toon(black_box(&data), black_box(&opts)).unwrap();
            decode_toon_to_json(
                black_box(&encoded),
                black_box(&DecodeOptions {
                    compact: true,
                    ..Default::default()
                }),
            )
            .unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_encode_text,
    bench_decode_text,
    bench_encode_compact,
    bench_decode_compact,
    bench_encode_tabular,
    bench_roundtrip_text,
    bench_roundtrip_compact
);
criterion_main!(benches);
