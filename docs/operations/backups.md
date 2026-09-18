# Operations — Backups & Baseline Preservation

This document details baseline preservation strategies, Content-Addressed Storage immutability, and cryptographic provenance ledgers.

---

## 1. Corpus Immutability Policy

The reference firmware corpus in `originals/` represents the ground truth for all system baselines:
- Files are accessed exclusively via read-only file handles (`SourceStore`).
- The filesystem permissions for `originals/` should remain read-only (`chmod -R 555 originals`).
- Regular hash audits verify that no baseline files have suffered bit-rot or accidental corruption:
  ```bash
  /usr/bin/python3 -c "import json, hashlib; print('Validating originals manifest...')"
  ```

---

## 2. Content-Addressed Storage (CAS) Snapshotting

Every binary asset ingested into the workstation is deduplicated and pinned into `.mmistudio/cas/`:
- Storage keys correspond to the 256-bit BLAKE3 cryptographic hash.
- Any modified asset is saved as an independent blob; original blobs remain indefinitely accessible.
- Staging references reference CAS blob IDs, ensuring that any stage workspace can be restored to its exact original state at any time.

---

## 3. Cryptographic Build Attestation

Whenever a candidate update is rebuilt, `mmi_attestation` records an immutable build manifest containing:
- Git commit hash of the workstation toolchain.
- SHA-256 hashes of all input assets and original firmware baselines.
- Exact declarative theme recipe ID and version.
- Rebuild timestamp and deterministic package checksums.
