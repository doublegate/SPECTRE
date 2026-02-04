# Docker Deployment

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Quick Start

### Pull Images

```bash
# SPECTRE CLI
docker pull ghcr.io/doublegate/spectre:latest

# CyberChef-MCP
docker pull ghcr.io/doublegate/cyberchef-mcp:latest
```

### Basic Usage

```bash
# Run SPECTRE
docker run --rm -it ghcr.io/doublegate/spectre:latest --version

# Scan (requires host networking)
docker run --rm -it --net=host --cap-add=NET_RAW \
    ghcr.io/doublegate/spectre:latest scan -sS 192.168.1.0/24
```

---

## Docker Compose

### Full Stack

```yaml
# docker-compose.yml
version: '3.8'

services:
  spectre:
    image: ghcr.io/doublegate/spectre:latest
    container_name: spectre
    network_mode: host
    cap_add:
      - NET_RAW
      - NET_ADMIN
    volumes:
      - ./config:/root/.config/spectre:ro
      - ./data:/root/.spectre
      - ./campaigns:/root/.spectre/campaigns
    environment:
      - SPECTRE_LOG_LEVEL=info
      - SPECTRE_CHEF_ENDPOINT=http://cyberchef:3000
    depends_on:
      - cyberchef
    stdin_open: true
    tty: true

  cyberchef:
    image: ghcr.io/doublegate/cyberchef-mcp:latest
    container_name: spectre-cyberchef
    ports:
      - "3000:3000"
    restart: unless-stopped

  # Optional: Web UI for results
  nginx:
    image: nginx:alpine
    ports:
      - "8080:80"
    volumes:
      - ./data/reports:/usr/share/nginx/html:ro
    depends_on:
      - spectre
```

### Start Services

```bash
docker-compose up -d

# Run scan
docker-compose exec spectre scan -sS 192.168.1.0/24

# View logs
docker-compose logs -f spectre
```

---

## Network Configuration

### Host Networking (Recommended for Scanning)

```bash
docker run --rm -it \
    --net=host \
    --cap-add=NET_RAW \
    --cap-add=NET_ADMIN \
    ghcr.io/doublegate/spectre:latest scan -sS target
```

### Bridge Networking

```bash
# Create network
docker network create spectre-net

# Run with bridge (limited scanning capability)
docker run --rm -it \
    --network spectre-net \
    ghcr.io/doublegate/spectre:latest scan -sT target
```

### Macvlan (Dedicated IP)

```bash
# Create macvlan network
docker network create -d macvlan \
    --subnet=192.168.1.0/24 \
    --gateway=192.168.1.1 \
    -o parent=eth0 spectre-macvlan

# Run with dedicated IP
docker run --rm -it \
    --network spectre-macvlan \
    --ip 192.168.1.200 \
    --cap-add=NET_RAW \
    ghcr.io/doublegate/spectre:latest scan -sS target
```

---

## Volume Mounts

### Configuration

```bash
# Mount config directory
docker run --rm -it \
    -v ~/.config/spectre:/root/.config/spectre:ro \
    ghcr.io/doublegate/spectre:latest
```

### Data Persistence

```bash
# Mount data directory
docker run --rm -it \
    -v ~/.spectre:/root/.spectre \
    ghcr.io/doublegate/spectre:latest
```

### Campaign Data

```bash
# Mount specific campaign directory
docker run --rm -it \
    -v ./campaigns:/root/.spectre/campaigns \
    ghcr.io/doublegate/spectre:latest
```

---

## Building Custom Image

### Dockerfile

```dockerfile
FROM rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libpcap-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpcap0.8 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/spectre /usr/local/bin/

RUN useradd -m spectre
USER spectre
WORKDIR /home/spectre

ENTRYPOINT ["spectre"]
```

### Build

```bash
docker build -t spectre:custom .
```

---

## Security Considerations

### Capability Minimization

```bash
# Only add required capabilities
docker run --rm -it \
    --cap-add=NET_RAW \
    --security-opt=no-new-privileges \
    ghcr.io/doublegate/spectre:latest
```

### Read-Only Root

```bash
docker run --rm -it \
    --read-only \
    --tmpfs /tmp \
    -v ~/.spectre:/root/.spectre \
    ghcr.io/doublegate/spectre:latest
```

### Resource Limits

```bash
docker run --rm -it \
    --memory=512m \
    --cpus=2 \
    --pids-limit=100 \
    ghcr.io/doublegate/spectre:latest
```

---

## Kubernetes Deployment

### Pod Specification

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: spectre
spec:
  hostNetwork: true
  containers:
  - name: spectre
    image: ghcr.io/doublegate/spectre:latest
    securityContext:
      capabilities:
        add: ["NET_RAW", "NET_ADMIN"]
    volumeMounts:
    - name: config
      mountPath: /root/.config/spectre
      readOnly: true
    - name: data
      mountPath: /root/.spectre
  volumes:
  - name: config
    configMap:
      name: spectre-config
  - name: data
    persistentVolumeClaim:
      claimName: spectre-data
```

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: spectre-config
data:
  spectre.toml: |
    [general]
    log_level = "info"

    [scan]
    rate = 1000
```

---

## Troubleshooting

### No Network Access

```bash
# Check network mode
docker inspect spectre | jq '.[0].HostConfig.NetworkMode'

# Must be "host" for raw socket access
```

### Permission Denied

```bash
# Check capabilities
docker inspect spectre | jq '.[0].HostConfig.CapAdd'

# Must include NET_RAW
```

### CyberChef Connection Failed

```bash
# Check container is running
docker ps | grep cyberchef

# Check network connectivity
docker exec spectre curl -s http://cyberchef:3000/health
```
