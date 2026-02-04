# Plugin API Reference

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE supports Lua 5.4 plugins for extending functionality. Plugins run in a sandboxed environment with controlled access to system resources.

---

## Plugin Structure

```
~/.spectre/plugins/
├── my-plugin/
│   ├── plugin.toml      # Plugin manifest
│   ├── init.lua         # Entry point
│   └── data/            # Plugin data files
```

### Plugin Manifest

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "Example plugin for SPECTRE"

[plugin.permissions]
network = false
filesystem = ["data/"]
```

---

## API Reference

### spectre.log

```lua
spectre.log.info("Message")
spectre.log.warn("Warning")
spectre.log.error("Error")
spectre.log.debug("Debug")
```

### spectre.scan

```lua
-- Start scan
local scan_id = spectre.scan.start({
    targets = {"192.168.1.0/24"},
    ports = "1-1000"
})

-- Get results
local results = spectre.scan.results(scan_id)
```

### spectre.chef

```lua
-- CyberChef operations
local decoded = spectre.chef.from_base64("SGVsbG8=")
local result = spectre.chef.bake("data", {"From_Base64", "Gunzip"})
```

### spectre.json

```lua
local str = spectre.json.encode({foo = "bar"})
local obj = spectre.json.decode('{"foo": "bar"}')
```

### spectre.crypto

```lua
local hash = spectre.crypto.sha256("data")
local hmac = spectre.crypto.hmac_sha256("key", "message")
```

### spectre.regex

```lua
local matches = spectre.regex.find_all(text, pattern)
local result = spectre.regex.replace(text, pattern, replacement)
```

---

## Event Hooks

```lua
function plugin.on_scan_start(scan)
    spectre.log.info("Scan started: " .. scan.id)
    return true
end

function plugin.on_scan_result(result)
    -- Process each result
    return result
end

function plugin.on_scan_complete(scan, results)
    spectre.log.info("Scan complete")
end
```

---

## Custom Service Probes

```lua
spectre.probe.register({
    name = "my-service",
    ports = {8000, 8001},
    protocol = "tcp",
    send = function() return "GET /\r\n" end,
    match = function(response)
        local ver = response:match("Version: ([%d.]+)")
        if ver then return {name = "MyService", version = ver} end
        return nil
    end
})
```

---

## Security

Plugins run in a sandboxed Lua environment:
- No direct system access
- Limited filesystem (plugin directory only)
- Network access requires permission
- Memory and CPU limits enforced
