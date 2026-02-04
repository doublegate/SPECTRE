# SITUATION REPORT (SITREP)

**Classification:** UNCLASSIFIED // FOR OFFICIAL USE ONLY

---

## Template

```
═══════════════════════════════════════════════════════════════════
                    SPECTRE SITUATION REPORT
═══════════════════════════════════════════════════════════════════

SITREP: [Campaign Codename] - [Sequential Number]
DTG:    [DDHHMMZ MMM YYYY]
FROM:   [Callsign/Operator]
TO:     [Distribution]

───────────────────────────────────────────────────────────────────
1. UNIT IDENTIFICATION
───────────────────────────────────────────────────────────────────
   Campaign:     [Name]
   Phase:        [Current Phase]
   Operator:     [Callsign]

───────────────────────────────────────────────────────────────────
2. ACTIVITY (Last Reporting Period)
───────────────────────────────────────────────────────────────────
   [Summary of actions taken]

   Tools Employed:
   - [ ] ProRT-IP    : [Activity summary]
   - [ ] CyberChef   : [Activity summary]
   - [ ] WRAITH      : [Activity summary]

───────────────────────────────────────────────────────────────────
3. TARGET STATUS
───────────────────────────────────────────────────────────────────
   Scope:        [IP ranges, domains, etc.]
   Hosts Found:  [Count]
   Services:     [Count]
   Vulns ID'd:   [Count]

───────────────────────────────────────────────────────────────────
4. OPERATIONAL STATUS
───────────────────────────────────────────────────────────────────
   Overall:      [GREEN/AMBER/RED]

   ┌─────────────┬────────┬─────────────────────────────┐
   │ Component   │ Status │ Notes                       │
   ├─────────────┼────────┼─────────────────────────────┤
   │ ProRT-IP    │        │                             │
   │ CyberChef   │        │                             │
   │ WRAITH      │        │                             │
   │ C2 Channel  │        │                             │
   └─────────────┴────────┴─────────────────────────────┘

   GREEN  = Fully operational
   AMBER  = Degraded / issues present
   RED    = Non-operational / blocked

───────────────────────────────────────────────────────────────────
5. SIGNIFICANT FINDINGS
───────────────────────────────────────────────────────────────────
   [List any notable discoveries, vulnerabilities, or intelligence]

   1. 
   2. 
   3. 

───────────────────────────────────────────────────────────────────
6. NEXT ACTIONS
───────────────────────────────────────────────────────────────────
   Immediate:    [Next 1-4 hours]
   Short-term:   [Next 24 hours]
   Dependencies: [Blockers or requirements]

───────────────────────────────────────────────────────────────────
7. REQUESTS/REQUIREMENTS
───────────────────────────────────────────────────────────────────
   [ ] None at this time
   [ ] Authorization expansion needed
   [ ] Additional resources required
   [ ] Guidance requested

   Details:

───────────────────────────────────────────────────────────────────
8. REMARKS
───────────────────────────────────────────────────────────────────
   [Additional context, observations, or concerns]

═══════════════════════════════════════════════════════════════════
                         END SITREP
═══════════════════════════════════════════════════════════════════
```

---

## Status Indicators

| Status | Meaning | Action Required |
|--------|---------|-----------------|
| 🟢 GREEN | On track, no issues | Continue as planned |
| 🟡 AMBER | Minor issues, degraded | Monitor, may need adjustment |
| 🔴 RED | Major issues, blocked | Immediate attention required |

---

## Frequency Guidelines

| Campaign Phase | SITREP Frequency |
|----------------|------------------|
| Initial Recon | Every 4 hours |
| Active Scanning | Every 2 hours |
| Exploitation | Every 1 hour |
| Exfiltration | Continuous |
| Wrap-up | Daily |

---

## Quick SITREP (Abbreviated)

For rapid updates when full format isn't practical:

```
QUICKSITREP [Campaign]-[#] @ [Time]
STATUS: [GREEN/AMBER/RED]
PHASE: [Current]
PROGRESS: [Brief summary]
NEXT: [Immediate action]
ISSUES: [None / Brief description]
```
