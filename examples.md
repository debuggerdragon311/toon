# TOON Examples

## Basic Usage

### Simple Object

```bash
# Input JSON
echo '{"name":"Alice","age":30,"active":true}' > person.json

# Encode to TOON text
toon encode person.json -o person.toon

# person.toon content:
# {
#   active: true
#   age: 30
#   name: Alice
# }

# Decode back
toon decode person.toon
```

### Arrays

```bash
# Array of primitives
echo '[1,2,3,4,5]' | toon encode

# Array of objects
echo '[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]' | toon encode --tabular-arrays
```

## Advanced Features

### Compact Binary Mode

Best for network transmission or storage:

```bash
# Encode to compact binary
toon encode large-data.json --compact -o data.toon

# Check size reduction
ls -lh large-data.json data.toon

# Decode (format auto-detected)
toon decode data.toon -o restored.json
```

### Tabular Arrays

For arrays of objects with identical keys:

```bash
# Create sample data
cat > users.json << 'EOF'
[
  {"id": 1, "name": "Alice", "email": "alice@example.com", "age": 30},
  {"id": 2, "name": "Bob", "email": "bob@example.com", "age": 25},
  {"id": 3, "name": "Charlie", "email": "charlie@example.com", "age": 35}
]
EOF

# Encode with tabular mode
toon encode users.json --tabular-arrays -o users.toon

# users.toon will contain:
# [
#   # age, email, id, name
#   30, alice@example.com, 1, Alice,
#   25, bob@example.com, 2, Bob,
#   35, charlie@example.com, 3, Charlie
# ]

# Decode preserves exact structure
toon decode users.toon -o users-restored.json
diff users.json users-restored.json  # Should be identical
```

### Strict Mode

Enforce validation:

```bash
# Non-uniform array with strict mode (will fail)
echo '[{"a":1},{"b":2}]' | toon encode --tabular-arrays --strict
# Error: Tabular mode requires uniform array of objects, but array has mixed types

# Works without strict (falls back to normal encoding)
echo '[{"a":1},{"b":2}]' | toon encode --tabular-arrays
```

## Real-World Scenarios

### API Response Processing

```bash
# Fetch data from API
curl https://api.github.com/repos/rust-lang/rust/issues?per_page=5 \
  | toon encode --compact > issues.toon

# Process later
toon decode issues.toon | jq '.[].title'
```

### Log File Compression

```bash
# Convert JSON logs to compact TOON
for log in logs/*.json; do
  toon encode "$log" --compact -o "${log%.json}.toon"
done

# Restore when needed
toon decode logs/app.toon | grep "ERROR"
```

### Data Pipeline

```bash
# Extract → Transform → Load pipeline
cat raw-data.json \
  | toon encode --tabular-arrays \
  | toon decode \
  | jq 'map(select(.active == true))' \
  | toon encode --compact \
  > filtered.toon
```

### Nested Structures

```bash
# Complex nested document
cat > complex.json << 'EOF'
{
  "metadata": {
    "version": "2.0",
    "created": "2024-01-01T00:00:00Z"
  },
  "users": [
    {
      "id": 1,
      "profile": {
        "name": "Alice",
        "settings": {
          "theme": "dark",
          "notifications": true
        }
      },
      "posts": [
        {"id": 101, "title": "Hello World"},
        {"id": 102, "title": "TOON is great"}
      ]
    }
  ]
}
EOF

# Encode preserving structure
toon encode complex.json -o complex.toon

# Decode and verify
toon decode complex.toon | jq '.users[0].profile.settings.theme'
# Output: "dark"
```

## Performance Tips

### Streaming Large Files

```bash
# Efficient for large files (streams through memory)
cat huge-dataset.json | toon encode --compact > huge.toon

# No need to load entire file into memory
toon decode huge.toon | head -n 100
```

### Batch Processing

```bash
# Process multiple files efficiently
find data/ -name "*.json" -print0 | \
  xargs -0 -I {} -P 4 sh -c 'toon encode "{}" --compact -o "{}.toon"'
```

## Edge Cases

### Empty Collections

```bash
echo '{"empty_array":[],"empty_object":{}}' | toon encode
# Output:
# {
#   empty_array: []
#   empty_object: {}
# }
```

### Special Characters

```bash
# Unicode and special chars preserved
echo '{"emoji":"🎉","unicode":"日本語","newline":"line1\nline2"}' \
  | toon encode | toon decode
```

### Numbers

```bash
# All number types supported
echo '{"int":42,"float":3.14,"exp":1.5e10,"negative":-42}' \
  | toon encode --compact | toon decode
```

## Integration Examples

### With jq

```bash
# TOON → JSON → jq → TOON pipeline
toon decode data.toon \
  | jq '.users | map(select(.age > 25))' \
  | toon encode --tabular-arrays \
  > filtered.toon
```

### With Python

```python
import subprocess
import json

# Encode with TOON
data = {"users": [{"id": 1, "name": "Alice"}]}
json_str = json.dumps(data)
result = subprocess.run(
    ["toon", "encode", "--compact"],
    input=json_str.encode(),
    capture_output=True
)
toon_data = result.stdout

# Decode with TOON
result = subprocess.run(
    ["toon", "decode"],
    input=toon_data,
    capture_output=True
)
restored = json.loads(result.stdout)
```

### As Git Filter

Add to `.gitattributes`:
```
*.toon filter=toon
```

Add to `.git/config`:
```
[filter "toon"]
    clean = toon encode --compact
    smudge = toon decode
```

## Testing Roundtrip

```bash
# Verify lossless roundtrip
echo '{"test":"data"}' > original.json
toon encode original.json | toon decode > restored.json
diff original.json restored.json  # Should have no differences

# Test with your own data
toon encode your-data.json | toon decode | diff your-data.json -
```
