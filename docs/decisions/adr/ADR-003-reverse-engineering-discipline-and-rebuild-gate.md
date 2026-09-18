# ADR-003: Reverse-Engineering Discipline and the Identity-Rebuild Gate

## Status
ACCEPTED [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L924]

## Date
2026-09-18 [EV:doc:PROJECT.md#L3]

## Context
Audi MMI update packages contain complex, undocumented binary containers created by Harman/Becker, Elektrobit, and QNX Software Systems [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348].
Modifying or repacking binary files without formal verification risks corrupting vehicle head units, leading to unrecoverable boot-loops or bricked display clusters [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830].
A rigorous, scientific discipline is necessary to ensure that [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348]:
1. Every unknown binary format is systematically documented and reverse-engineered with declarative specifications [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352].
2. Any newly implemented format codec can decode and re-encode original files without introducing bit rot or pixel distortion [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
3. Signed firmware images are protected against modification, while unsigned visual and language resources can be cleanly modified [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].

## Decision
Establish the **Reverse-Engineering Research Register (RQ-REGISTER)**, **Kaitai Struct Specifications**, and the **Identity-Rebuild Gate** as mandatory quality gates [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348]:
1. **RQ-REGISTER & Kaitai Struct**: Every proprietary format is tracked in `docs/research/RQ-REGISTER.md` and specified declaratively in `formats/*.ksy` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L352]. No format may enter write pipelines without resolving its RQ [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L364].
2. **Identity-Rebuild Gate (`mmi-formats::gate`)**: Prior to deploying any modified asset, the original binary must pass through round-trip re-encoding [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368]:
   - `PassedBitForBit`: Decoded and re-encoded payload matches the original binary bit-for-bit (SHA-256 identical) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L369].
   - `PassedCanonical`: Canonical structures and payloads match identically, with harmless differences accounted for (such as zlib header timestamp flags) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L370].
   - `Failed`: Any discrepancy in dimensions, colour channels, or data headers immediately halts the pipeline with `canRebuild = NO` [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L371].
3. **Signed Artefact Immutability**: All files with detached `.sig` signatures (firmware IFS/EFS, kernel images, navigation routing graphs) are permanently locked against modification (`canEdit = NO`, `canRebuild = NO`) [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].

## Alternatives Considered

### Ad-hoc Binary Patching (Hex Offsets)
- Pros: Simple to write one-off shell scripts or hex editor offsets [INF:HIGH basis: common hobbyist ECU modification].
- Cons: Extremely fragile across firmware releases; minor version bumps shift offsets, causing catastrophic flash corruption [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348].
- Rejected: Violates §13.1 mandate prohibiting hardcoded byte offsets [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L348].

### Best-Effort Re-encoding without Identity Gate
- Pros: Easier to implement converters without strict round-trip verification [INF:MEDIUM basis: rapid prototyping].
- Cons: Compression differences or color space shifts (e.g. RGB vs BGR) can remain undetected until flashing onto real vehicle hardware [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L368].
- Rejected: Unacceptable risk of rendering artifacts or vehicle hardware crashes [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L830].

## Consequences
- 100% of the 12 proprietary formats discovered in `originals/` are formally documented and tested [EV:tree@originals/8R0051884KL_6.36.0_2023/pkgdb/LIT/EJ211Ga_L1.db].
- Zero visual degradation: cluster graphics re-encode bit-for-bit with exact 32-bit RGBA pixel retention [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L369].
- Strict safety enforcement guarantees signed OS and DSP binaries cannot be inadvertently modified [EV:doc:AUDI_MMI_STUDIO_AGENT_PROMPT.md#L103].
