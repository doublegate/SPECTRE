# Tutorial: Campaign Planning

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

This tutorial demonstrates how to plan and execute multi-phase security assessment campaigns using SPECTRE.

**Time Required:** 30 minutes

**Prerequisites:**
- SPECTRE installed
- Familiarity with scanning and analysis
- Written authorization for target scope

---

## Step 1: Create a Campaign

```bash
spectre campaign create "Network Assessment Q1"
```

Output:
```
Campaign created: camp_abc123
Name: Network Assessment Q1
Created: 2026-02-04T10:00:00Z

Campaign directory: ~/.spectre/campaigns/camp_abc123/
```

---

## Step 2: Define Scope

Add targets to the campaign:

```bash
# Add network ranges
spectre campaign scope add 192.168.1.0/24
spectre campaign scope add 10.0.0.0/24

# Add specific hosts
spectre campaign scope add webserver.example.com

# Import from file
spectre campaign scope import targets.txt

# Set exclusions
spectre campaign scope exclude 192.168.1.1   # Gateway
spectre campaign scope exclude 192.168.1.254 # Critical
```

View scope:
```bash
spectre campaign scope list
```

---

## Step 3: Plan Phases

Create campaign phases:

```bash
# Phase 1: Discovery
spectre campaign phase create discovery \
    --description "Host discovery and port scanning"

# Phase 2: Enumeration
spectre campaign phase create enumeration \
    --description "Service detection and vulnerability scanning"

# Phase 3: Analysis
spectre campaign phase create analysis \
    --description "Data analysis and reporting"
```

---

## Step 4: Execute Discovery Phase

Run host discovery:

```bash
# Start phase
spectre campaign phase start discovery

# Discover hosts
spectre scan -sn 192.168.1.0/24 --campaign

# Fast port scan
spectre scan -sS -F --campaign
```

Review results:
```bash
spectre campaign status
spectre campaign findings
```

---

## Step 5: Execute Enumeration Phase

Deep scan discovered hosts:

```bash
# Start phase
spectre campaign phase start enumeration

# Service detection
spectre scan -sS -sV --campaign

# OS detection
spectre scan -sS -O --campaign

# Specific service enumeration
spectre scan -sS -sV -p 80,443,8080 --script http-enum --campaign
```

---

## Step 6: Analyze Data

Process findings with CyberChef:

```bash
# Start analysis phase
spectre campaign phase start analysis

# Extract all banners
spectre campaign export banners | spectre chef "Extract_URLs,Extract_IP_addresses"

# Analyze for patterns
spectre campaign analyze
```

---

## Step 7: Workflow Automation

Create automated workflow:

```bash
spectre workflow create recon-basic << 'EOF'
name: Basic Reconnaissance
description: Standard network reconnaissance workflow

phases:
  - name: discovery
    steps:
      - scan: -sn {targets}
      - scan: -sS -F {targets}

  - name: enumeration
    steps:
      - scan: -sS -sV -p- {hosts_up}
      - scan: -sS -O {hosts_up}

  - name: analysis
    steps:
      - chef: Extract_URLs,Extract_IP_addresses
      - report: generate
EOF
```

Run workflow:
```bash
spectre workflow run recon-basic --targets 192.168.1.0/24 --campaign
```

---

## Step 8: Generate Reports

Create campaign report:

```bash
# Summary report
spectre campaign report summary

# Detailed report
spectre campaign report detailed --format pdf

# Export findings
spectre campaign export findings --format json > findings.json
spectre campaign export findings --format csv > findings.csv
```

---

## Step 9: Campaign Commands

```bash
# Campaign status
spectre campaign status

# List all campaigns
spectre campaign list

# Switch campaigns
spectre campaign use camp_xyz789

# Pause campaign
spectre campaign pause

# Resume campaign
spectre campaign resume

# Complete campaign
spectre campaign complete

# Archive campaign
spectre campaign archive
```

---

## Using Military Briefing Templates

SPECTRE includes military-style briefing templates:

### OPORD (Operations Order)

```bash
# Generate OPORD
spectre campaign opord generate > opord.md
```

Template sections:
1. Situation
2. Mission
3. Execution
4. Sustainment
5. Command and Control

### SITREP (Situation Report)

```bash
# Generate SITREP
spectre campaign sitrep > sitrep.md
```

### AAR (After Action Review)

```bash
# Generate AAR at campaign completion
spectre campaign aar > aar.md
```

---

## Campaign Configuration

```toml
# spectre.toml
[campaign]
data_dir = "~/.spectre/campaigns"
auto_save = 60
max_artifacts = 10000

[campaign.defaults]
# Default phases for new campaigns
phases = ["discovery", "enumeration", "exploitation", "analysis"]

# Automatic findings deduplication
deduplicate = true

# Archive completed campaigns
auto_archive = true
archive_after_days = 30
```

---

## Best Practices

1. **Authorization First** - Always have written authorization
2. **Document Everything** - Use OPORD, SITREP, AAR templates
3. **Phase Discipline** - Complete each phase before proceeding
4. **Backup Regularly** - Export campaign data frequently
5. **Review Before Proceeding** - Analyze findings between phases

---

## Next Steps

- [OPORD Template](../briefings/OPORD-template.md) - Operations order format
- [CLI Reference](../user-guide/CLI-REFERENCE.md) - Complete command reference
- [Operational Security](../security/OPERATIONAL-SECURITY.md) - OPSEC guidelines
