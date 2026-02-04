# Tutorial: Data Analysis with CyberChef

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

This tutorial demonstrates using SPECTRE's CyberChef integration for data analysis and transformation.

**Time Required:** 15 minutes

**Prerequisites:**
- SPECTRE installed
- CyberChef-MCP running (`spectre chef setup`)

---

## Step 1: Basic Operations

### Encoding/Decoding

```bash
# Base64 decode
spectre chef "From_Base64" --input "SGVsbG8gV29ybGQh"
# Output: Hello World!

# Hex encode
spectre chef "To_Hex" --input "Hello"
# Output: 48 65 6c 6c 6f

# URL decode
spectre chef "URL_Decode" --input "Hello%20World"
# Output: Hello World
```

### Hashing

```bash
# SHA256
spectre chef "SHA2" --input "test data"
# Output: 916f0027...

# Multiple hashes
spectre chef "MD5,SHA1,SHA2" --input "test"
```

---

## Step 2: Chaining Operations (Recipes)

Chain multiple operations together:

```bash
# Decode base64, then decompress
spectre chef "From_Base64,Gunzip" --input "H4sIAAAA..."

# Decode, decrypt, extract
spectre chef "From_Base64,AES_Decrypt({key:'...'}),Extract_URLs" --input "..."
```

---

## Step 3: Extract IOCs

Extract indicators of compromise:

```bash
# Extract from text
spectre chef "Extract_URLs" --input "Visit https://example.com for more"

# Extract IPs
spectre chef "Extract_IP_addresses" --file log.txt

# Extract all IOCs
spectre chef "Extract_URLs,Extract_IP_addresses,Extract_domains,Extract_email_addresses" \
    --file malware_report.txt
```

---

## Step 4: Analyze Scan Results

Process scan output:

```bash
# Extract banners from scan results
spectre scan -sS -sV 192.168.1.0/24 -o json | \
    jq -r '.hosts[].open_ports[].banner' | \
    spectre chef "Extract_URLs,Unique"

# Analyze service versions
spectre scan -sV 192.168.1.1 -o json | \
    spectre chef "JPath_expression({path:'$.hosts[*].open_ports[*].version'})"
```

---

## Step 5: Magic Detection

Auto-detect encoding:

```bash
# Detect format
spectre chef magic --input "UEsDBBQAAAA..."
# Output: ZIP archive (95% confidence)

# Auto-decode
spectre chef magic --auto --input "data..."
```

---

## Step 6: Working with Files

Process files:

```bash
# Analyze binary
spectre chef "Entropy,Strings" --file suspicious.bin

# Extract from archive
spectre chef "Unzip" --file archive.zip --output extracted/

# Calculate file hash
spectre chef "SHA2" --file document.pdf
```

---

## Step 7: Save Recipes

Save frequently used recipes:

```bash
# Save recipe
spectre chef recipe save "decode-credentials" "From_Base64,URL_Decode"

# Use saved recipe
spectre chef "@decode-credentials" --input "..."

# List recipes
spectre chef recipe list

# Export recipe
spectre chef recipe export "decode-credentials" > recipe.json
```

---

## Common Recipes

### Decode Obfuscated PowerShell

```bash
spectre chef "From_Base64,Decode_text('UTF-16LE')" \
    --input "JABzAD0ATgBlAHcA..."
```

### Analyze Base64 + Gzip

```bash
spectre chef "From_Base64,Gunzip,Strings" --input "H4sI..."
```

### Extract Network IOCs

```bash
spectre chef "Extract_URLs,Extract_IP_addresses,Defang_URL,Defang_IP_addresses,Unique" \
    --file threat_intel.txt
```

### Decode JWT

```bash
spectre chef "JWT_Decode" --input "eyJhbGc..."
```

---

## Step 8: Batch Processing

Process multiple inputs:

```bash
# From file list
cat encoded_strings.txt | spectre chef "From_Base64" --batch

# Process directory
spectre chef "SHA2" --input-dir samples/ --output hashes.txt
```

---

## Integration with Scans

### Post-Scan Analysis Pipeline

```bash
# 1. Run scan
spectre scan -sS -sV 192.168.1.0/24 -o json > scan.json

# 2. Extract and analyze banners
cat scan.json | \
    jq -r '.hosts[].open_ports[].banner // empty' | \
    spectre chef "Extract_URLs,Extract_IP_addresses,Unique"

# 3. Decode any encoded data
cat scan.json | \
    jq -r '.hosts[].open_ports[].banner // empty' | \
    grep -E '^[A-Za-z0-9+/]+=*$' | \
    spectre chef "From_Base64"
```

---

## Available Operations

```bash
# List all operations
spectre chef --list

# Search operations
spectre chef --list | grep -i base64

# Operation details
spectre chef --describe "From_Base64"
```

Categories:
- **Encoding:** Base64, Hex, URL, HTML entities
- **Encryption:** AES, DES, XOR, RSA
- **Hashing:** MD5, SHA, BLAKE
- **Compression:** Gzip, Deflate, Bzip2, Zip
- **Extraction:** URLs, IPs, emails, strings
- **Parsing:** JSON, XML, CSV
- **Analysis:** Entropy, frequency, diff

---

## Next Steps

- [MCP Tools Reference](../user-guide/MCP-TOOLS.md) - AI assistant integration
- [Plugin API](../api/PLUGIN-API.md) - Extend with custom operations
