# ADR-002: Tauri Desktop Architecture and Native Processing Boundary

## Status
ACCEPTED [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924]

## Date
2026-09-18 [EV:doc:PROJECT.md#L3]

## Context
Audi MMI Studio requires a modern graphical user interface providing interactive asset board exploration, 800x480 screen composition, true typography rendering, recipe editing, and package rebuilding [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L465].
The engineering environment enforces strict technical constraints [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L745]:
- High performance on large binaries (up to 25 GB archives, 63.97 GiB total corpus) [EV:survey@scan_originals.py].
- Zero telemetry and strictly offline execution [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L814].
- Minimal memory footprint and low native latency [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L807].
- Clean separation between graphical user interface presentation and native binary parsing [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L756].

## Decision
Adopt **Tauri v2** with a React 19 + TypeScript + Vite frontend and Rust native processing backend [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L748]:
1. **Frontend Presentation**: React 19 + TypeScript with Tailwind CSS and Konva 2D canvas for high-speed virtualized asset boards and screen composition [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L754].
2. **Backend Processing**: Rust native workspace handling all filesystem operations, binary Kaitai parsing, image decoding/encoding, cryptographic hashing, and media creation [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L756].
3. **IPC Boundary**: Strongly typed Tauri IPC commands serialize lightweight domain entities between Rust and TypeScript via JSON, with zero binary decoding performed in JavaScript [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L756].
4. **Offline and Portability**: Zero remote CDN dependencies; all assets, icons, and fonts bundled locally [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L826].

## Alternatives Considered

### Electron
- Pros: Mature ecosystem, ubiquitous documentation [INF:HIGH basis: general desktop framework knowledge].
- Cons: Bundles entire Chromium runtime and Node.js; excessive memory footprint (~200+ MB idle); poor fit for memory-constrained reverse-engineering environments [INF:HIGH basis: framework comparison metrics].
- Rejected: Heavyweight resource consumption violates performance budget (§17.3) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L807].

### Pure Native GUI (Iced / Slint / egui)
- Pros: 100% Rust, zero web view overhead [INF:HIGH basis: Rust GUI ecosystem].
- Cons: Immature component ecosystem for complex diffing (Monaco), rich layout rendering, and complex typography canvas integration [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L753].
- Rejected: Slows development velocity for rich visual asset inspectors [INF:MEDIUM basis: engineering ergonomics].

## Consequences
- The workstation achieves native execution speeds with <30 MB idle memory overhead [INF:HIGH basis: Tauri v2 memory profile].
- All binary parsing, hashing, and security gates remain strictly encapsulated in Rust crates [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L756].
- Desktop UI communicates exclusively through structured IPC commands matching headless CLI operations [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L788].
