# Glossary

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## A

**ACK Scan**
: TCP scan that sends ACK packets to determine firewall rulesets and identify filtered/unfiltered ports.

**AF_XDP**
: Address Family XDP - Linux kernel technology for high-performance packet processing with kernel bypass.

**Artifact**
: Any data item collected or generated during a campaign (scan results, captured files, reports).

---

## B

**Banner**
: Text response returned by a service when a connection is made, often revealing version information.

**Banner Grabbing**
: Technique to collect service banners for version identification.

---

## C

**C2 (Command and Control)**
: Infrastructure and protocols used to communicate with and control compromised systems.

**Campaign**
: Organized security assessment consisting of multiple phases and coordinated activities.

**CIDR**
: Classless Inter-Domain Routing - notation for IP address ranges (e.g., 192.168.1.0/24).

**Connect Scan**
: TCP scan that completes the full three-way handshake.

**CyberChef**
: Web application for data analysis and transformation, integrated via MCP in SPECTRE.

---

## D

**Decoy Scanning**
: Technique to obscure the true source of scans by generating traffic from spoofed IPs.

**Double Ratchet**
: Cryptographic protocol providing forward secrecy and post-compromise security.

---

## E

**E2EE (End-to-End Encryption)**
: Encryption where only communicating parties can read messages.

**Enumeration**
: Phase of assessment focused on gathering detailed information about discovered services.

---

## F

**FIN Scan**
: TCP scan sending FIN packets to identify open ports (no response = open).

**Finding**
: Discovered vulnerability, misconfiguration, or notable observation during assessment.

**Forward Secrecy**
: Property ensuring past communications remain secure if long-term keys are compromised.

---

## G

**Greppable Output**
: Output format designed for easy parsing with grep and other text tools.

---

## H

**Honeypot**
: Decoy system designed to detect and analyze attacks.

**Host Discovery**
: Process of identifying live hosts on a network.

---

## I

**IDS (Intrusion Detection System)**
: System that monitors network traffic for suspicious activity.

**IOC (Indicator of Compromise)**
: Artifact indicating potential security breach (IP, URL, hash, etc.).

**io_uring**
: Linux asynchronous I/O interface for high-performance operations.

---

## K

**Key Exchange**
: Protocol for securely establishing shared cryptographic keys.

---

## L

**Lua**
: Scripting language used for SPECTRE plugins.

---

## M

**MCP (Model Context Protocol)**
: Protocol for AI assistant integration with external tools.

**Mimicry**
: Technique to make protocol traffic appear as legitimate traffic.

---

## N

**Noise Protocol**
: Framework for building cryptographic protocols, used in WRAITH.

**Nmap**
: Popular network scanner; SPECTRE uses compatible syntax.

**NULL Scan**
: TCP scan sending packets with no flags set.

---

## O

**OPORD (Operations Order)**
: Military-style document for mission planning.

**OPSEC (Operational Security)**
: Practices to protect sensitive information during operations.

**OS Fingerprinting**
: Identifying operating system by analyzing network behavior.

---

## P

**Ping Sweep**
: Host discovery using ICMP echo requests.

**Port**
: Network endpoint identified by number (1-65535).

**Post-Quantum**
: Cryptography designed to resist quantum computer attacks.

**ProRT-IP**
: SPECTRE's network reconnaissance component.

---

## R

**Ratchet**
: Cryptographic mechanism for key evolution.

**Recipe**
: Sequence of CyberChef operations.

**Red Team**
: Security team simulating adversary attacks.

**Recon (Reconnaissance)**
: Information gathering phase of assessment.

---

## S

**Service Detection**
: Identifying services running on open ports.

**SITREP (Situation Report)**
: Status update during operations.

**SYN Scan**
: TCP scan sending only SYN packets (half-open).

---

## T

**Target**
: System or network being assessed.

**Timing Template**
: Preset scan timing configuration (T0-T5).

**TUI (Terminal User Interface)**
: Text-based graphical interface in terminal.

---

## U

**UDP Scan**
: Scan for UDP services (connectionless protocol).

---

## W

**WRAITH**
: SPECTRE's secure communication protocol component.

---

## X

**X25519**
: Elliptic curve Diffie-Hellman key exchange.

**XChaCha20-Poly1305**
: Authenticated encryption algorithm.

**Xmas Scan**
: TCP scan with FIN, PSH, and URG flags set.

---

## Acronyms

| Acronym | Expansion |
|---------|-----------|
| AAR | After Action Review |
| ACK | Acknowledgment |
| AF_XDP | Address Family Express Data Path |
| API | Application Programming Interface |
| C2 | Command and Control |
| CIDR | Classless Inter-Domain Routing |
| CLI | Command Line Interface |
| E2EE | End-to-End Encryption |
| GUI | Graphical User Interface |
| IDS | Intrusion Detection System |
| IOC | Indicator of Compromise |
| MCP | Model Context Protocol |
| OPORD | Operations Order |
| OPSEC | Operational Security |
| SITREP | Situation Report |
| SYN | Synchronize |
| TCP | Transmission Control Protocol |
| TUI | Terminal User Interface |
| UDP | User Datagram Protocol |
