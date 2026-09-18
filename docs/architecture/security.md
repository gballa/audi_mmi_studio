# Architecture — Security & Threat Modeling

This document details the security architecture, STRIDE threat model, trust boundaries, and Egress Airlock implemented in **Audi MMI Studio**.

---

## 1. STRIDE Threat Model

| STRIDE Threat Category | Potential Attack Vector | Platform Mitigation Architecture |
| :--- | :--- | :--- |
| **Spoofing** | Forging update manifests to install unauthenticated code on vehicle head unit. | Identity-Rebuild Gate permanently locks signed binaries (`.pkg.sig`, IFS bootloaders); all rebuilt packages emit cryptographic attestation manifests (`mmi_attestation`). |
| **Tampering** | Modifying immutable reference corpus or inserting unvetted dependencies. | `SourceStore` enforces strict read-only access to `originals/`; all cargo builds run strictly offline (`--offline`) using pinned checksums. |
| **Repudiation** | Untracked modifications causing vehicle system instability. | Declarative JSON theme recipes maintain cryptographic journal ledgers with SHA-256 state hashes for all transformations. |
| **Information Disclosure** | Leakage of vehicle identification numbers (VIN), proprietary tokens, or firmware bytes. | Zero-telemetry policy; desktop webview locked to `tauri://localhost`; AI prompt sanitization strips VINs, serials, and proprietary brand terms before reaching the Egress Airlock. |
| **Denial of Service** | Malformed container causing workstation crash or head-unit brick during flashing. | Bounds-checked safe Rust decoders; plugin memory allocation capped at 64 MB; QNX pre-flight simulator verifies partition capacities before SD card media deployment. |
| **Elevation of Privilege** | Untrusted format adapter plugin attempting system privilege escalation. | Plugins run in restricted sandboxes without network access, filesystem writes, or process spawning capabilities. |

---

## 2. Three-Tier Trust Boundary System

```text
┌─────────────────────────────────────────────────────────────┐
│  Tier 1: Untrusted External Boundary (Egress Airlock)       │
│  - Prompt sanitization, brand redaction, offline mock       │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│  Tier 2: Workstation Processing Boundary (Rust Crates)      │
│  - Safe byte slice decoders, memory-bounded plugin sandbox  │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│  Tier 3: Immutable Vehicle Reference Boundary (originals/)  │
│  - SourceStore read-only lock; signed asset rebuild lock    │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. Egress Airlock Architecture (`mmi_imagegen`)

To prevent accidental data exfiltration during AI asset generation:
1. **Prompt Sanitization**: Prompts are scanned for sensitive strings (VIN, chassis numbers, component serials, proprietary brand marks). Any matching prompt is rejected immediately.
2. **Key Isolation**: Tokens are stored strictly in OS keychain memory and never serialized to project files or git logs.
3. **Offline Mock Fallback**: In offline workstation mode (`MMI_OFFLINE_MOCK=true`), all requests route to a local procedural mock generator with zero external network connectivity.
