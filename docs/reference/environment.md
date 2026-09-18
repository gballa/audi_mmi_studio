# Reference — Environment Variables & Isolation Boundaries

This document defines the runtime environment variables, filesystem security boundaries, and sandbox controls in **Audi MMI Studio**.

---

## 1. Environment Variables

| Variable | Default Value | Description |
| :--- | :--- | :--- |
| `CARGO_NET_OFFLINE` | `true` | When set to `true`, Cargo is prohibited from attempting any network connections during compilation or testing. |
| `RUST_LOG` | `info` | Configures the `tracing-subscriber` logging filter. Valid values: `trace`, `debug`, `info`, `warn`, `error`. |
| `MMI_STUDIO_DIR` | `.mmistudio` | Root path for workstation application data, CAS storage, and staging caches. |
| `MMI_ORIGINALS_DIR` | `originals` | Root path to the immutable reference firmware corpus. |
| `MMI_OFFLINE_MOCK` | `true` | When `true`, all external AI generation requests in `mmi-imagegen` route to the deterministic local mock provider. |

---

## 2. Filesystem Access Boundaries

The workstation enforces strict three-tier filesystem boundaries:

```text
Tier 1: Read-Only Corpus (originals/)
  - Permission: READ-ONLY
  - Enforcement: SourceStore abstraction returns error on any mutating operation.

Tier 2: Workstation Data (.mmistudio/)
  - Permission: READ / WRITE
  - Enforcement: Scoped to workspace root; CAS blobs indexed by BLAKE3 hash.

Tier 3: Output Artifacts (output/)
  - Permission: WRITE-ONLY / OVERWRITE
  - Enforcement: Rebuilt trees, SD images, and attestation manifests.
```

---

## 3. Network Isolation Guarantees

In accordance with specification §17.4 and ADR-001:
- **Zero Inbound Ports**: The platform opens no listening TCP/UDP sockets.
- **Zero Outbound Telemetry**: Zero crash reporters, analytics pings, or cloud update checks.
- **Webview Origin Binding**: The Tauri v2 desktop webview is locked to `tauri://localhost`. Remote script tags, iframes, and CORS external requests are blocked by Content Security Policy.
