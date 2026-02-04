# CyberChef-MCP Integration

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

CyberChef-MCP provides SPECTRE's data analysis and transformation capabilities, offering 463+ operations for encoding, decoding, encryption, hashing, extraction, and data manipulation.

**Component Version:** v1.8.0
**Repository:** [github.com/doublegate/CyberChef-MCP](https://github.com/doublegate/CyberChef-MCP)

---

## Capabilities

| Feature | Description |
|---------|-------------|
| **Operations** | 463+ CyberChef operations exposed via MCP |
| **Recipes** | Multi-step processing pipelines |
| **Batch Processing** | Process multiple inputs in parallel |
| **Format Support** | Text, binary, hex, base64, files |
| **Streaming** | Process large files without loading entirely |
| **Magic Detection** | Auto-detect encoding/compression |

---

## Integration Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                    SPECTRE ↔ CyberChef-MCP Integration                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                        SPECTRE Core                                     │ │
│  │                                                                         │ │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐ │ │
│  │  │   Chef Manager  │  │  Recipe Store   │  │   Results Cache         │ │ │
│  │  │                 │  │                 │  │                         │ │ │
│  │  │  • Job queue    │  │  • Saved recipes│  │  • LRU cache            │ │ │
│  │  │  • Batch mgmt   │  │  • Templates    │  │  • Disk persistence     │ │ │
│  │  │  • Progress     │  │  • Sharing      │  │  • Deduplication        │ │ │
│  │  └────────┬────────┘  └────────┬────────┘  └────────────┬────────────┘ │ │
│  │           └───────────────────┬┴────────────────────────┘              │ │
│  └───────────────────────────────┼────────────────────────────────────────┘ │
│                                  │                                          │
│  ┌───────────────────────────────▼────────────────────────────────────────┐ │
│  │                    CyberChef Integration Layer                          │ │
│  │                                                                         │ │
│  │  ┌────────────────────────────────────────────────────────────────────┐ │ │
│  │  │                    SpectreChef API                                  │ │ │
│  │  │                                                                    │ │ │
│  │  │   fn bake(input, recipe) -> BakeResult                             │ │ │
│  │  │   fn bake_stream(input, recipe) -> impl Stream<ChunkResult>        │ │ │
│  │  │   fn magic(input) -> Vec<DetectedFormat>                           │ │ │
│  │  │   fn list_operations() -> Vec<Operation>                           │ │ │
│  │  │   fn recipe_save(name, recipe) -> ()                               │ │ │
│  │  │   fn recipe_load(name) -> Recipe                                   │ │ │
│  │  └────────────────────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────┬────────────────────────────────────────┘ │
│                                  │                                          │
│  ┌───────────────────────────────▼────────────────────────────────────────┐ │
│  │                      MCP Protocol Layer                                 │ │
│  │                                                                         │ │
│  │  ┌───────────────────────────────────────────────────────────────────┐  │ │
│  │  │                    MCP Client                                      │  │ │
│  │  │                                                                   │  │ │
│  │  │  • JSON-RPC transport (stdio/HTTP)                                │  │ │
│  │  │  • Tool invocation                                                 │  │ │
│  │  │  • Resource management                                             │  │ │
│  │  │  • Connection pooling                                              │  │ │
│  │  └───────────────────────────────────────────────────────────────────┘  │ │
│  └───────────────────────────────┬────────────────────────────────────────┘ │
│                                  │                                          │
│  ┌───────────────────────────────▼────────────────────────────────────────┐ │
│  │                    CyberChef-MCP Server                                 │ │
│  │                                                                         │ │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐   │ │
│  │  │   Encoding    │  │   Crypto      │  │      Extraction           │   │ │
│  │  │               │  │               │  │                           │   │ │
│  │  │  • Base64     │  │  • AES/DES    │  │  • URLs, IPs              │   │ │
│  │  │  • Hex        │  │  • XOR        │  │  • Domains, emails        │   │ │
│  │  │  • URL        │  │  • RSA        │  │  • Files, strings         │   │ │
│  │  │  • HTML       │  │  • Hashing    │  │  • Regex patterns         │   │ │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘   │ │
│  │                                                                         │ │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐   │ │
│  │  │  Compression  │  │   Parsing     │  │      Analysis             │   │ │
│  │  │               │  │               │  │                           │   │ │
│  │  │  • Gzip       │  │  • JSON       │  │  • Entropy                │   │ │
│  │  │  • Deflate    │  │  • XML        │  │  • Frequency              │   │ │
│  │  │  • Bzip2      │  │  • CSV        │  │  • Diff                   │   │ │
│  │  │  • Zlib       │  │  • YAML       │  │  • Statistics             │   │ │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘   │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## API Reference

### SpectreChef

Main interface for CyberChef operations within SPECTRE.

```rust
use spectre_chef::{SpectreChef, ChefConfig};

// Initialize chef (connects to CyberChef-MCP)
let chef = SpectreChef::new(ChefConfig {
    mcp_endpoint: "stdio://cyberchef-mcp".into(),
    timeout: Duration::from_secs(30),
    ..Default::default()
}).await?;
```

### Single Operation

```rust
// Base64 decode
let result = chef.from_base64("SGVsbG8gV29ybGQh").await?;
println!("{}", result);  // "Hello World!"

// Hex encode
let result = chef.to_hex("Hello", HexOptions {
    delimiter: Delimiter::Space,
    ..Default::default()
}).await?;
println!("{}", result);  // "48 65 6c 6c 6f"

// Hash
let result = chef.sha256("test data").await?;
println!("{}", result);  // "916f0027..."
```

### Recipe Execution

```rust
use spectre_chef::{Recipe, Operation};

// Build recipe programmatically
let recipe = Recipe::new()
    .add(Operation::FromBase64 { alphabet: None })
    .add(Operation::Gunzip)
    .add(Operation::ExtractUrls { unique: true });

let result = chef.bake("H4sIAAAA...", &recipe).await?;

// Or use inline syntax
let result = chef.bake_inline(
    input,
    "From_Base64,Gunzip,Extract_URLs"
).await?;
```

### Batch Processing

```rust
use spectre_chef::BatchOptions;

// Process multiple inputs
let inputs = vec![
    "SGVsbG8=",
    "V29ybGQ=",
    "VGVzdA==",
];

let results = chef.bake_batch(
    &inputs,
    &Recipe::new().add(Operation::FromBase64 { alphabet: None }),
    BatchOptions {
        parallel: true,
        max_concurrent: 10,
        ..Default::default()
    },
).await?;

for (input, output) in inputs.iter().zip(results.iter()) {
    println!("{} -> {}", input, output);
}
```

### Streaming Large Files

```rust
use futures::StreamExt;

let mut stream = chef.bake_stream(
    File::open("large_file.bin")?,
    &Recipe::new()
        .add(Operation::Gunzip)
        .add(Operation::ExtractStrings { min_length: 4 }),
).await?;

while let Some(chunk) = stream.next().await {
    let result = chunk?;
    // Process chunk results
}
```

### Magic Detection

```rust
let detections = chef.magic("UEsDBBQAAAA...").await?;

for detection in detections {
    println!("{}: {} ({}% confidence)",
        detection.format,
        detection.description,
        detection.confidence
    );
}
// Output: "ZIP: ZIP archive (95% confidence)"
```

---

## Operation Categories

### Encoding/Decoding

| Operation | Description |
|-----------|-------------|
| `From_Base64` / `To_Base64` | Base64 encoding/decoding |
| `From_Hex` / `To_Hex` | Hexadecimal encoding/decoding |
| `URL_Decode` / `URL_Encode` | URL percent encoding |
| `HTML_Entity_Decode` / `Encode` | HTML entity encoding |
| `From_Binary` / `To_Binary` | Binary string conversion |
| `From_Octal` / `To_Octal` | Octal conversion |
| `From_Morse_Code` / `To_Morse_Code` | Morse code |
| `ROT13` / `ROT47` | ROT encoding |

```rust
// Chain encodings
let recipe = Recipe::new()
    .add(Operation::FromBase64 { alphabet: None })
    .add(Operation::UrlDecode { plus_as_space: true })
    .add(Operation::HtmlEntityDecode);
```

### Compression

| Operation | Description |
|-----------|-------------|
| `Gunzip` / `Gzip` | Gzip compression |
| `Inflate` / `Deflate` | Raw deflate |
| `Unzip` / `Zip` | ZIP archives |
| `Bzip2_Decompress` / `Compress` | Bzip2 |
| `Zlib_Inflate` / `Deflate` | Zlib wrapper |
| `LZMA_Decompress` / `Compress` | LZMA |

```rust
// Decompress nested archives
let recipe = Recipe::new()
    .add(Operation::FromBase64 { alphabet: None })
    .add(Operation::Gunzip)
    .add(Operation::Unzip { password: None });
```

### Hashing

| Operation | Description |
|-----------|-------------|
| `MD5` | MD5 hash (128-bit) |
| `SHA1` | SHA-1 hash (160-bit) |
| `SHA2` | SHA-256, SHA-384, SHA-512 |
| `SHA3` | SHA3-256, SHA3-512 |
| `BLAKE2b` / `BLAKE2s` | BLAKE2 variants |
| `BLAKE3` | BLAKE3 hash |
| `HMAC` | Keyed-hash MAC |

```rust
// Generate multiple hashes
let recipe = Recipe::new()
    .add(Operation::Md5)
    .add(Operation::Register { name: "md5".into() })
    .add(Operation::Input)  // Reset to original
    .add(Operation::Sha256)
    .add(Operation::Register { name: "sha256".into() });
```

### Encryption/Decryption

| Operation | Description |
|-----------|-------------|
| `AES_Encrypt` / `Decrypt` | AES-128/192/256 |
| `DES_Encrypt` / `Decrypt` | DES/3DES |
| `Blowfish_Encrypt` / `Decrypt` | Blowfish |
| `XOR` / `XOR_Brute_Force` | XOR operations |
| `RC4` | RC4 stream cipher |
| `RSA_Encrypt` / `Decrypt` | RSA asymmetric |

```rust
use spectre_chef::{AesMode, AesPadding};

let decrypted = chef.aes_decrypt(
    ciphertext,
    AesOptions {
        key: hex::decode("000102030405...")?,
        iv: hex::decode("101112131415...")?,
        mode: AesMode::CBC,
        padding: AesPadding::Pkcs7,
    },
).await?;
```

### Extraction

| Operation | Description |
|-----------|-------------|
| `Extract_URLs` | Find URLs in text |
| `Extract_IP_addresses` | IPv4 and IPv6 |
| `Extract_domains` | Domain names |
| `Extract_email_addresses` | Email addresses |
| `Extract_file_paths` | File system paths |
| `Extract_dates` | Date patterns |
| `Strings` | Extract printable strings |
| `Regular_expression` | Custom regex extraction |

```rust
// Extract IOCs from malware analysis
let recipe = Recipe::new()
    .add(Operation::ExtractUrls { unique: true })
    .add(Operation::Register { name: "urls".into() })
    .add(Operation::Input)
    .add(Operation::ExtractIpAddresses {
        include_ipv6: true,
        remove_local: true
    })
    .add(Operation::Register { name: "ips".into() });
```

### Parsing

| Operation | Description |
|-----------|-------------|
| `JSON_Beautify` / `Minify` | JSON formatting |
| `XML_Beautify` / `Minify` | XML formatting |
| `Parse_CSV` | CSV to JSON |
| `Parse_YAML` | YAML parsing |
| `Parse_ASN.1` | ASN.1 structure |
| `Parse_X.509` | Certificate parsing |
| `Parse_TLV` | TLV data parsing |

```rust
// Parse and extract from JSON
let recipe = Recipe::new()
    .add(Operation::JsonBeautify { indent: 2 })
    .add(Operation::JPathExpression {
        path: "$.data[*].value".into()
    });
```

### Analysis

| Operation | Description |
|-----------|-------------|
| `Entropy` | Calculate entropy |
| `Frequency_distribution` | Byte frequency |
| `Diff` | Compare inputs |
| `Detect_file_type` | Magic bytes detection |
| `Chi_squared` | Statistical analysis |
| `Index_of_coincidence` | Crypto analysis |

```rust
// Analyze suspicious file
let entropy = chef.entropy(data).await?;
println!("Entropy: {:.2} bits/byte", entropy);

if entropy > 7.5 {
    println!("Likely encrypted or compressed");
}
```

### Data Manipulation

| Operation | Description |
|-----------|-------------|
| `Find_/_Replace` | Search and replace |
| `Filter` | Filter lines by pattern |
| `Sort` | Sort lines |
| `Unique` | Remove duplicates |
| `Reverse` | Reverse string/bytes |
| `Split` / `Merge` | Split and merge data |
| `Head` / `Tail` | First/last N items |

```rust
// Clean and deduplicate IOCs
let recipe = Recipe::new()
    .add(Operation::ExtractUrls { unique: false })
    .add(Operation::Filter {
        regex: "^https?://".into(),
        invert: false
    })
    .add(Operation::Sort { descending: false })
    .add(Operation::Unique { case_sensitive: false });
```

---

## Configuration

### SPECTRE Config (spectre.toml)

```toml
[chef]
# MCP server endpoint
mcp_endpoint = "stdio://cyberchef-mcp"

# Alternative: HTTP endpoint
# mcp_endpoint = "http://localhost:3000"

# Operation timeout (seconds)
timeout = 30

# Maximum input size (MB)
max_input_size = 100

# Enable result caching
cache_enabled = true
cache_size = 1000  # Number of results to cache

[chef.recipes]
# Recipe storage directory
directory = "~/.spectre/recipes"

# Auto-save recipes
auto_save = true

[chef.batch]
# Default batch concurrency
max_concurrent = 10

# Batch timeout multiplier
timeout_multiplier = 2.0
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `SPECTRE_CHEF_ENDPOINT` | Override MCP endpoint |
| `SPECTRE_CHEF_TIMEOUT` | Override timeout |
| `CYBERCHEF_MCP_LOG` | MCP server log level |

---

## Recipe Management

### Saving Recipes

```rust
// Save for later use
chef.recipe_save(
    "decode-credentials",
    &Recipe::new()
        .add(Operation::FromBase64 { alphabet: None })
        .add(Operation::UrlDecode { plus_as_space: true }),
    RecipeMetadata {
        description: "Decode common credential encodings".into(),
        tags: vec!["credentials", "decode"],
        ..Default::default()
    },
).await?;
```

### Loading Recipes

```rust
// Load saved recipe
let recipe = chef.recipe_load("decode-credentials").await?;
let result = chef.bake(input, &recipe).await?;

// Or use @ syntax in CLI/inline
let result = chef.bake_inline(input, "@decode-credentials").await?;
```

### Listing Recipes

```rust
let recipes = chef.recipe_list().await?;
for recipe in recipes {
    println!("{}: {}", recipe.name, recipe.description);
}
```

### Sharing Recipes

```rust
// Export to file
chef.recipe_export("decode-credentials", "recipe.json").await?;

// Import from file
chef.recipe_import("recipe.json").await?;

// Import from URL
chef.recipe_import_url("https://example.com/recipes/decode.json").await?;
```

### Built-in Recipes

SPECTRE includes common security analysis recipes:

| Recipe | Description |
|--------|-------------|
| `@decode-credentials` | Decode Base64/URL encoded credentials |
| `@extract-iocs` | Extract URLs, IPs, domains, emails |
| `@decode-powershell` | Decode PowerShell encoded commands |
| `@analyze-malware` | Extract strings, entropy analysis |
| `@decode-jwt` | Parse and decode JWT tokens |
| `@network-iocs` | Extract network indicators |

---

## Integration with Scan Results

### Processing Banners

```rust
// Extract information from service banners
let scan_results = scanner.scan_syn(&targets, ports, opts).await?;

for host in scan_results.hosts {
    for port in host.open_ports {
        if let Some(banner) = &port.banner {
            // Extract version information
            let versions = chef.bake_inline(
                banner,
                "Regular_expression(/(\\d+\\.\\d+\\.\\d+)/g)"
            ).await?;

            // Check for sensitive information
            let sensitive = chef.bake_inline(
                banner,
                "Extract_email_addresses,Extract_file_paths"
            ).await?;
        }
    }
}
```

### Building IOC Database

```rust
// Process all scan data for IOCs
let all_banners: Vec<&str> = scan_results.hosts
    .iter()
    .flat_map(|h| h.open_ports.iter())
    .filter_map(|p| p.banner.as_deref())
    .collect();

let iocs = chef.bake_batch(
    &all_banners,
    &chef.recipe_load("@extract-iocs").await?,
    BatchOptions::default(),
).await?;
```

---

## Error Handling

### Operation Errors

```rust
match chef.bake(input, &recipe).await {
    Ok(result) => println!("Success: {}", result),
    Err(ChefError::InvalidInput(msg)) => {
        eprintln!("Invalid input: {}", msg);
    }
    Err(ChefError::OperationFailed { op, reason }) => {
        eprintln!("Operation '{}' failed: {}", op, reason);
    }
    Err(ChefError::Timeout) => {
        eprintln!("Operation timed out");
    }
    Err(ChefError::McpConnection(e)) => {
        eprintln!("MCP connection error: {}", e);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Partial Results

```rust
let result = chef.bake_lenient(input, &recipe).await?;

if !result.warnings.is_empty() {
    for warning in &result.warnings {
        eprintln!("Warning: {}", warning);
    }
}

println!("Output: {}", result.output);
```

---

## Performance Optimization

### Caching

```rust
// Enable caching for repeated operations
let chef = SpectreChef::new(ChefConfig {
    cache_enabled: true,
    cache_size: 1000,
    ..Default::default()
}).await?;

// Same input + recipe will return cached result
let result1 = chef.bake("same input", &recipe).await?;
let result2 = chef.bake("same input", &recipe).await?;  // Cached
```

### Streaming for Large Files

```rust
// Don't load entire file into memory
let result = chef.bake_file(
    Path::new("large_file.bin"),
    &recipe,
    StreamOptions {
        chunk_size: 1024 * 1024,  // 1 MB chunks
        ..Default::default()
    },
).await?;
```

### Batch vs Sequential

```rust
// Batch is faster for many small inputs
let results = chef.bake_batch(&many_inputs, &recipe, BatchOptions {
    parallel: true,
    max_concurrent: 20,
    ..Default::default()
}).await?;

// Sequential is better for few large inputs
for input in few_large_inputs {
    let result = chef.bake(input, &recipe).await?;
}
```

---

## MCP Server Management

### Starting the Server

```bash
# SPECTRE auto-starts CyberChef-MCP as needed
spectre chef "From_Base64" --input data.txt

# Or start manually
cyberchef-mcp serve --port 3000
```

### Docker Deployment

```bash
# Run CyberChef-MCP in Docker
docker run -d --name cyberchef-mcp \
    -p 3000:3000 \
    ghcr.io/doublegate/cyberchef-mcp:latest

# Configure SPECTRE to use it
export SPECTRE_CHEF_ENDPOINT="http://localhost:3000"
```

### Health Check

```rust
let status = chef.health_check().await?;
println!("Server: {} ({})", status.version, status.status);
println!("Operations: {} available", status.operation_count);
```

---

## Troubleshooting

### MCP Connection Failed

```bash
# Check if server is running
pgrep cyberchef-mcp

# Test connection
spectre chef --health

# Check logs
tail -f ~/.spectre/logs/chef.log

# Try HTTP endpoint instead of stdio
export SPECTRE_CHEF_ENDPOINT="http://localhost:3000"
```

### Operation Not Found

```bash
# List all available operations
spectre chef --list-operations

# Search for operation
spectre chef --list-operations | grep -i base64

# Check operation details
spectre chef --describe From_Base64
```

### Invalid Input

```bash
# Check input encoding
file input.txt
hexdump -C input.txt | head

# Try magic detection
spectre chef "Detect_file_type" --input input.bin

# Use appropriate decoding first
spectre chef "From_Hex,From_Base64" --input data.txt
```

### Timeout Errors

```bash
# Increase timeout
spectre chef --timeout 120 "Heavy_operation" --input large.bin

# Use streaming for large files
spectre chef --stream "Gunzip" --input huge.gz
```

---

## References

- [CyberChef-MCP README](https://github.com/doublegate/CyberChef-MCP/blob/main/README.md)
- [CyberChef Operations](https://gchq.github.io/CyberChef/)
- [MCP Protocol Specification](https://modelcontextprotocol.io/docs)
- [CyberChef GitHub](https://github.com/gchq/CyberChef)
