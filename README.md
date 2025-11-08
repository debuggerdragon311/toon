# TOON - A Compact, Lossless JSON Encoding Format

TOON is a space-efficient, token-efficient alternative to JSON that maintains perfect round-trip compatibility. It offers both human-readable text mode and binary compact mode, with special optimizations for tabular data.

## Why TOON?

- **Lossless**: Perfect round-trip to and from JSON
- **Compact**: Reduced size compared to JSON (especially in compact mode)
- **LLM-Friendly**: Fewer tokens for language models
- **Fast**: Streaming support with O(n) parsing
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Type-safe**: Preserves all JSON types (null, bool, number, string, array, object)

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
git clone https://github.com/toon-format/toon
cd toon
cargo build --release
```

## Usage

### Basic Encoding/Decoding

```bash
# Encode JSON to TOON (text mode)
echo '{"name":"Alice","age":30}' | toon encode > data.toon

# Decode TOON back to JSON
toon decode data.toon

# Use files instead of stdin/stdout
toon encode input.json -o output.toon
toon decode output.toon -o output.json
```

### Compact Binary Mode

```bash
# Encode to compact binary format
toon encode data.json --compact -o data.toon

# Decode from compact format (auto-detected)
toon decode data.toon -o data.json
```

### Tabular Arrays Mode

For arrays of objects with identical keys, TOON can use a space-efficient tabular layout:

```bash
# Input: [{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]
toon encode users.json --tabular-arrays -o users.toon

# Output uses header row + value rows
# [
#   # id, name
#   1, Alice,
#   2, Bob
# ]
```

### Options

- `--compact`: Use binary length-prefixed format for maximum compression
- `--tabular-arrays`: Optimize uniform arrays of objects
- `--indent <n>`: Set indentation (default: 2 spaces)
- `--strict`: Fail on validation errors or non-uniform arrays
- `-o, --out <file>`: Output file (default: stdout)

## Format Specifications

### TOON-Text (Default)

An indentation-based format that removes braces and minimizes quotes:

```toon
{
  name: Alice
  age: 30
  active: true
  tags: [
    important,
    urgent
  ]
}
```

### TOON-Compact (Binary)

Length-prefixed binary format:
- Magic header: `TOON\x01`
- Type tags: 1 byte per value
- Strings: u32 length + UTF-8 bytes
- Arrays/Objects: u32 count + elements

### TOON-Tabular

For uniform arrays of objects:
```toon
[
  # id, name, score
  1, Alice, 100,
  2, Bob, 95,
  3, Charlie, 88
]
```

## Guarantees

- **Lossless round-trip**: JSON → TOON → JSON produces identical output
- **Array order preserved**: Elements stay in original order
- **Object keys sorted**: For deterministic output (encoding only)
- **Type safety**: All JSON types supported (null, bool, number, string, array, object)
- **Streaming**: O(n) parsing with constant memory overhead
- **No data loss**: Numbers, unicode, nested structures all preserved

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run property tests (may take longer)
cargo test --release

# Run benchmarks
cargo bench
```

## Performance

TOON is designed for:
- Streaming stdin/stdout workflows
- Zero-copy parsing where possible
- Minimal allocations
- Fast round-trip encoding/decoding

Benchmarks on typical JSON documents show:
- Text mode: 10-30% size reduction
- Compact mode: 30-50% size reduction
- Tabular mode: 40-60% size reduction (for uniform arrays)

## Error Handling

TOON provides clear, contextual error messages:

```bash
$ toon decode malformed.toon
Error: Failed to decode text TOON
  Caused by: Expected ':' after object key at line 2
```

Exit codes:
- `0`: Success
- `1`: Parse or I/O error
- `2`: Validation error (strict mode)

## Examples

### Complex Document

```bash
# Input JSON
cat > users.json << 'EOF'
{
  "users": [
    {"id": 1, "name": "Alice", "email": "alice@example.com"},
    {"id": 2, "name": "Bob", "email": "bob@example.com"}
  ],
  "metadata": {
    "version": "1.0",
    "created": "2024-01-01"
  }
}
EOF

# Encode with tabular arrays
toon encode users.json --tabular-arrays > users.toon

# Decode back
toon decode users.toon -o users-restored.json

# Verify
diff users.json users-restored.json
```

### Piping and Filters

```bash
# Fetch JSON from API and convert to TOON
curl https://api.example.com/data.json | toon encode --compact > data.toon

# Convert back when needed
toon decode data.toon | jq '.users[] | select(.active == true)'
```

## Contributing

Contributions welcome! Please ensure:
- All tests pass: `cargo test`
- Code is formatted: `cargo fmt`
- No clippy warnings: `cargo clippy`
- Add tests for new features

## License

MIT

