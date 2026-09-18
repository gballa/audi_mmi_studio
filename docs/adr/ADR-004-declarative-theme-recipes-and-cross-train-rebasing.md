# ADR-004: Declarative Theme Recipes and Cross-Train Rebasing

## Status
ACCEPTED [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924]

## Date
2026-09-18 [EV:doc:PROJECT.md#L3]

## Context
Vehicle customizers and automotive engineers desire customized instrument cluster icons, modern color schemes, and updated language tables for MMI 3G+ systems [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L554].
However, update trains diverge across model years (e.g. `K0942_4`, `P0922`, `K0900`) and regional markets (`EU`, `US`, `NAR`, `CN`) [EV:doc:SOURCE_AUDIT.md#L30].
Directly distributing patched binary files across disparate software trains creates severe hazards [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L588]:
- Firmware version mismatch errors (`Error 140`, `Error 160`) during head unit update [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830].
- Accidental overwriting of vehicle-specific variant coding [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L588].
- Inability to audit or revert changes made by third-party modifications [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L572].

## Decision
Adopt **Declarative JSON Theme Recipes (`mmi-recipe`)** as the universal unit of modification and distribution [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L555]:
1. **Semantic Selectors**: Recipes target logical entities (`AssetPath`, `ModuleId`, `StringKey`, `ConfigKey`) rather than fragile physical byte offsets [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L560].
2. **Three-Tier Risk Classification**: Every operation is tagged with intrinsic risk (`Cosmetic`, `Content`, `Structural`) to alert users to safety implications [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L566].
3. **Cryptographic Audit Journal (`JournalChain`)**: All mutations append SHA-256 and BLAKE3 hash pairs into a tamper-evident cryptographic chain [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L572].
4. **Rebase Engine (`mmi_recipe::RebaseEngine`)**: Before applying a recipe to a different software train, the engine evaluates compatibility, detecting clean matches, path drift, or conflicting structures [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L588].

## Alternatives Considered

### Binary Delta Patches (IPS / BPS / BSDiff)
- Pros: Compact diff representation of altered bytes [INF:HIGH basis: ROM hacking standards].
- Cons: Blind byte replacements fail when target files differ by even 1 byte, leading to silent vehicle crashes [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L555].
- Rejected: Unsafe for cross-train automotive deployments [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L588].

### Imperative Bash / Python Shell Scripts
- Pros: Easy to write sequential copy and sed commands [INF:HIGH basis: script automation].
- Cons: Imperative scripts lack formal schema validation, cannot be previewed in GUI canvas, and lack deterministic rollback guarantees [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L555].
- Rejected: Fails reproducibility and safety requirements (§13) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L555].

## Consequences
- Theme modifications are completely portable, inspectable, and human-readable JSON documents [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L555].
- Users can safely evaluate whether a theme created for `K0942_4` can be rebased onto `P0922` prior to writing SD media [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L588].
- Rebuilding produces fully attested SD media volumes with instant emergency rollback capability [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830].
