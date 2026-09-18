# ADR-001: Offline-First Modular Rust Workspace Architecture

## Status
ACCEPTED [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924]

## Date
2026-09-18 [EV:doc:PROJECT.md#L3]

## Context
Audi MMI Studio is designed to inspect, analyze, reverse-engineer, and safely repackage software updates for Audi MMI 3G+ (HN+/HN+R) infotainment head units [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L6].
The project operates under stringent engineering constraints [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L926]:
- **Corpus Integrity**: The genuine reference corpus in `originals/` comprises 63.97 GiB and 24,676 files that must remain strictly immutable and read-only [EV:survey@scan_originals.py].
- **Zero Telemetry and Airgap Compliance**: Automotive reverse engineering workflows must run 100% offline without remote network egress, phone-home analytics, or external CDN dependencies [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].
- **High Determinism and Cryptographic Provenance**: Software mutations must be reproducible bit-for-bit with cryptographic audit trails (SHA-256 and BLAKE3) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L930].
- **Memory Safety and Performance**: Processing multi-gigabyte files (such as 3.38 GiB routing databases and 14.1 GiB ATLAS tile archives) requires zero-cost abstractions, bounded memory buffers, and strong concurrency [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L807].

## Decision
Adopt a modular, multi-crate Rust workspace comprising 14 decoupled crates and an offline-first Tauri 2 desktop GUI layer [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L771]:
1. **Immutable Storage Layer (`mmi-core`)**: Encapsulate file access behind `SourceStore` and `ImmutablePath` primitives that enforce read-only semantics at the type system level [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L926].
2. **Decoupled Binary Format Adapters (`mmi-formats`)**: Separate reverse-engineered binary codecs (`.precomp`, `.xar`, `.db`, `.atlas`, `.ifs`, `.efs`, `.ans`, `.hbbin`, etc.) from higher-level business logic [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L774].
3. **Dedicated Reverse-Engineering Lab (`mmi-re-lab`)**: Provide isolated primitives for sliding Shannon entropy profiling, byte histograms, signature carving, and string extraction [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L776].
4. **Isolated Egress Airlock (`mmi-imagegen`)**: Strictly isolate external network requests behind prompt sanitization, VIN redaction, and Content-Addressed Storage (CAS) pinning [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L939].
5. **Headless CLI Parity (`mmi-studio-cli`)**: Ensure 100% of workstation features are executable via headless CLI commands for scripted, offline CI/CD environments [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L788].

## Alternatives Considered

### Monolithic Python Script Suite
- Pros: Rapid prototyping and rich scientific libraries [INF:HIGH basis: Python ecosystem].
- Cons: Lacks compile-time memory safety, suffers performance bottlenecks on 20+ GB archives, and runtime dependency management complicates airgap distribution [INF:HIGH basis: Python deployment overhead].
- Rejected: Fails performance and determinism requirements (§17) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L807].

### C++20 Workstation
- Pros: Maximum performance and native ABI compatibility with embedded targets [INF:HIGH basis: C++ systems programming].
- Cons: Manual memory management increases risk of memory corruption vulnerabilities when parsing untrusted, fuzz-tested automotive archives [INF:HIGH basis: memory safety statistics].
- Rejected: Violates robust error-recovery and fuzz-safety requirements (§18) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L818].

### Single Monolithic Rust Crate
- Pros: Simpler initial scaffolding with fewer `Cargo.toml` manifests [INF:MEDIUM basis: project setup ergonomics].
- Cons: Degrades build times, impairs code isolation, and prevents selective reuse by third-party plugin developers [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L771].
- Rejected: Violates modular workspace mandate (§15.1) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L771].

## Consequences
- Strict boundary isolation guarantees `originals/` cannot be modified even under fatal error conditions [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L926].
- All builds and tests run with `--offline` flag without network access [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].
- High compilation speeds through incremental crate caching across the 14 workspace members [INF:HIGH basis: Cargo compilation model].
