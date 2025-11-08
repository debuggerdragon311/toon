# TOON Project Structure

```
toon/
├── Cargo.toml                 # Package manifest with dependencies
├── README.md                  # Main documentation
├── LICENSE-MIT                # MIT license
├── LICENSE-APACHE             # Apache 2.0 license
├── EXAMPLES.md                # Usage examples
├── PROJECT_STRUCTURE.md       # This file
│
├── .github/
│   └── workflows/
│       └── ci.yml            # GitHub Actions CI pipeline
│
├── src/
│   ├── lib.rs                # Public API and core types
│   ├── main.rs               # Binary entry point
│   ├── cli.rs                # Command-line argument parsing
│   ├── encoder.rs            # JSON → TOON encoding logic
│   ├── decoder.rs            # TOON → JSON decoding logic
│   │
│   └── codec/
│       ├── mod.rs            # Codec module exports
│       ├── text.rs           # TOON-Text format implementation
│       ├── compact.rs        # TOON-Compact binary format
│       └── tabular.rs        # Tabular array optimization
│
├── tests/
│   ├── roundtrip.rs          # Comprehensive roundtrip tests
│   ├── tabular.rs            # Tabular mode tests
│   └── property.rs           # Property-based tests with proptest
│
└── benches/
    └── encode_decode.rs      # Criterion benchmarks
```

## Module Responsibilities

### Core Library (`src/lib.rs`)

Defines the public API:
- `EncodeOptions` - Configuration for encoding
- `DecodeOptions` - Configuration for decoding  
- `encode_json_to_toon()` - Main encoding function
- `decode_toon_to_json()` - Main decoding function

### CLI Module (`src/cli.rs`)

Handles command-line interaction:
- Argument parsing with clap
- File I/O (stdin/stdout or file paths)
- Error reporting
- Exit code management

### Encoder (`src/encoder.rs`)

Orchestrates encoding:
- Routes to appropriate codec (text/compact/tabular)
- Validates options
- Handles strict mode enforcement

### Decoder (`src/decoder.rs`)

Orchestrates decoding:
- Auto-detects format from magic headers
- Routes to appropriate codec
- Error context propagation

### Text Codec (`src/codec/text.rs`)

TOON-Text format:
- Indentation-based syntax
- Minimal quoting (only when needed)
- Human-readable output
- Lossless to JSON

### Compact Codec (`src/codec/compact.rs`)

TOON-Compact binary format:
- Magic header: `TOON\x01`
- Type tags for each value
- Length-prefixed strings
- Count-prefixed collections
- Deterministic encoding (sorted keys)

### Tabular Codec (`src/codec/tabular.rs`)

Tabular array optimization:
- Detects uniform arrays of objects
- Emits header row once
- Streams value rows
- Falls back gracefully for non-uniform data

## Test Organization

### Roundtrip Tests (`tests/roundtrip.rs`)

Comprehensive fixture-based tests:
- All JSON types (null, bool, number, string, array, object)
- Empty collections
- Nested structures
- Array order preservation
- Large documents
- Both text and compact modes

### Tabular Tests (`tests/tabular.rs`)

Tabular mode validation:
- Uniform arrays (success case)
- Non-uniform arrays (fallback)
- Strict mode enforcement
- Edge cases (empty, single element)
- Nested values in tabular cells

### Property Tests (`tests/property.rs`)

Generative testing with proptest:
- Random JSON value generation
- Bounded depth and size
- Roundtrip invariant checking
- Both text and compact modes

## Build Artifacts

### Debug Build
```bash
cargo build
# Creates: target/debug/toon
```

### Release Build
```bash
cargo build --release
# Creates: target/release/toon (optimized)
```

### Tests
```bash
cargo test                    # Run all tests
cargo test --release          # With optimizations
cargo test roundtrip          # Specific test module
```

### Benchmarks
```bash
cargo bench                   # Run all benchmarks
cargo bench encode_text       # Specific benchmark
```

## Key Design Decisions

### No Unsafe Code
All code is safe Rust - no `unsafe` blocks used anywhere.

### Error Handling
Uses `anyhow::Result` throughout for rich error context.

### Deterministic Output
Object keys are sorted during encoding for reproducibility.

### Streaming Support
Parsers work in O(n) time with minimal memory overhead.

### Cross-Platform
No platform-specific APIs - works on Linux, macOS, Windows.

### Zero Dependencies on Runtime
Binary has no external library dependencies at runtime.

## Performance Characteristics

### Time Complexity
- Encoding: O(n) where n = input size
- Decoding: O(n) where n = input size
- Tabular detection: O(n·k) where k = object keys

### Space Complexity
- Text mode: ~1.2x input size (during encoding)
- Compact mode: ~0.7x input size (output)
- Tabular mode: ~0.5x for uniform arrays

### Optimization Opportunities
1. **Zero-copy string handling** - Use `Cow<str>` for borrowed strings
2. **Buffered I/O** - Already implemented via stdin/stdout
3. **SIMD parsing** - Future: Use SIMD for number parsing
4. **Parallel encoding** - Future: Split large arrays across threads

## Extension Points

### Adding New Codecs
1. Create `src/codec/your_format.rs`
2. Implement `encode()` and `decode()` functions
3. Add routing in `encoder.rs` and `decoder.rs`
4. Add tests in `tests/`

### Adding CLI Flags
1. Add field to `Commands` enum in `src/cli.rs`
2. Add to `EncodeOptions` or `DecodeOptions`
3. Update help text
4. Add tests

### Format Evolution
Use version bytes in magic headers:
- `TOON\x01` - Current compact format
- `TOON\x02` - Future enhanced format
- `TOON-TAB\x01` - Tabular format

## CI/CD Pipeline

GitHub Actions runs:
1. **Build** - On Linux, Windows, macOS
2. **Test** - All test suites
3. **Format** - `cargo fmt` check
4. **Clippy** - Lint warnings as errors

## Documentation Standards

### Code Comments
- Public APIs: Full doc comments with examples
- Internal functions: Brief description of purpose
- Complex algorithms: Step-by-step explanation

### README
- What, Why, How structure
- Installation instructions
- Usage examples
- API guarantees

### EXAMPLES.md
- Real-world use cases
- Integration patterns
- Edge cases
- Performance tips

## Release Checklist

Before releasing:
1. ✅ All tests pass (`cargo test`)
2. ✅ No clippy warnings (`cargo clippy`)
3. ✅ Formatted (`cargo fmt`)
4. ✅ Benchmarks run (`cargo bench`)
5. ✅ README updated
6. ✅ Version bumped in Cargo.toml
7. ✅ CHANGELOG updated
8. ✅ Git tagged

## Future Enhancements

Potential additions:
- [ ] Schema validation mode
- [ ] Streaming API for large files
- [ ] Compression integration (zstd, lz4)
- [ ] WASM build for browser use
- [ ] Language bindings (Python, Node.js)
- [ ] VSCode extension for .toon files
