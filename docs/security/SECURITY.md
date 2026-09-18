# Audi MMI Studio — Security and Hardening Architecture

This document formalizes the security architecture, threat model, trust boundaries, and hardening controls enforced across Audi MMI Studio [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924].

---

## 1. Threat Model & Trust Boundaries (STRIDE)

Audi MMI Studio operates at the boundary between untrusted automotive firmware archives, untrusted user-supplied assets, and mission-critical vehicle head-unit hardware [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L6].

| Threat Category | Potential Attack Vector | Applied Workstation Mitigation |
|---|---|---|
| **Spoofing** | Forged update packages or altered cryptographic checksums | Strict BLAKE3 and SHA-256 cryptographic verification; signed payload protection blocks spoofed firmware [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103] |
| **Tampering** | Modifying signed operating system binaries (`.ifs`, `.efs`) or DSP microcode | Type-level `SourceStore` immutability and `ERR_SIGNED_ARTEFACT_IMMUTABLE` hard block on detached `.sig` files [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L117] |
| **Repudiation** | Unaudited modifications causing vehicle malfunctions without provenance | Cryptographic audit journaling (`JournalChain`) records every operation with input/output hash pairs [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L572] |
| **Information Disclosure** | Leakage of vehicle telemetry, VIN numbers, or proprietary binary code | Strict Zero-Telemetry Policy (§17.4) and VIN redaction airlock on any generative AI prompt [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L939] |
| **Denial of Service** | Malformed archives or decompression bombs crashing workstation or head unit | Memory-bounded Kaitai parsers, Lanczos3 resize clamps, and 6-tier pre-flight simulation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L818] |
| **Elevation of Privilege** | Plugin breakout or unauthorized vehicle variant recoding | Sandboxed plugin architecture (`mmi-plugin`) with strict memory limits and restricted filesystem access [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L787] |

---

## 2. The Three-Tier Security Boundary System

### Tier 1: Invariant Controls (Always Enforced, No Exceptions)
- **Reference Corpus Immutability**: The 63.97 GiB genuine reference corpus in `originals/` is encapsulated behind `SourceStore` and `ImmutablePath` primitives that reject write operations at the type level [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L926].
- **Signed Artefact Hard Lock**: Any file accompanied by a detached cryptographic signature (`.pkg.sig`, `.dat.sig`) is permanently classified `ANALYSIS-ONLY` (`canEdit = NO`, `canRebuild = NO`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
- **Zero-Telemetry Guarantee**: Zero remote telemetry, zero analytics tracking, and zero crash reporting are compiled into workstation binaries [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L948].
- **Fuzz Safety & Panic Freedom**: Binary decoders are hardened to return structured `Result<T, CoreError>` errors without panicking on corrupted, truncated, or malicious payloads [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L818].

### Tier 2: Guarded Controls (Explicit Validation & Evidence Required)
- **Egress Airlock (`mmi-imagegen`)**: All network connectivity is disabled by default and restricted exclusively to AI asset generation through an egress filter that sanitizes prompts and redacts vehicle identifiers [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L939].
- **Identity-Rebuild Gate (`mmi-formats`)**: Modified assets must prove round-trip canonical or bit-for-bit fidelity prior to entering deployment media packaging [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
- **Pre-Flight Simulation (`mmi-media`)**: Update structures are dry-run simulated against QNX checksum verification models before physical media flashing [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L708].

### Tier 3: Permanent Prohibitions (Forbidden Operations)
- **Banned Safety Assertion (§14.9)**: The string `"SAFE TO INSTALL"` is permanently banned from all outputs, manifests, CLI logs, and documentation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L742].
- **No In-Vehicle Communication**: The workstation generates offline FAT32 SD media images but never initiates direct vehicle bus or OBD-II communications [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830].
- **No Activation / Licensing Bypass**: Materials concerning software licensing or copyright bypass (`6.22.4 Vlasoff`, `License/`) are classified `OUT OF SCOPE — DOCUMENT ONLY` [EV:doc:SOURCE_AUDIT.md#L78].

---

## 3. Mandatory Safety Vocabulary

All build manifests and validation reports strictly emit standardized status vocabulary [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737]:
- `VERIFIED`: Confirmed bit-for-bit or canonically identical through identity gate [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `SUPPORTED`: Formally analyzed format with automated decode and repackage capabilities [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `PARTIALLY SUPPORTED`: Read-only extraction verified; write or edit pipeline disabled [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `EXPERIMENTAL`: Codec under active research; restricted to developer testing [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `UNSUPPORTED`: Unmapped container structure; modification blocked [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `UNKNOWN`: Candidate binary without recognized file signature [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `RESEARCH REQUIRED`: Tracked under active research question in `docs/research/RQ-REGISTER.md` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `BUILD READY — DEPLOYMENT NOT VERIFIED`: Package rebuilt deterministically; vehicle installation remains unverified user risk [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `SIMULATED — NOT A GUARANTEE`: Package passed dry-run simulation without hardware warranty [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `PROTECTED / OUT OF SCOPE — DOCUMENT ONLY`: Signed payload or licensing material locked against mutation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
- `HIGH RISK — NO VERIFIED RECOVERY PATH`: Operation carries severe risk of unrecoverable hardware bricking [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L737].
