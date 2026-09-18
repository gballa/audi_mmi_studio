# Development — Release Packaging & Distribution

This document details the release process, standalone binary compilation, checksum generation, and Software Bill of Materials (SBOM) specifications for **Audi MMI Studio**.

---

## 1. Release Packaging Workflow

```bash
# 1. Ensure working directory is clean and tests pass
./scripts/verify-workstation.sh

# 2. Build optimized release binary
cargo build --release -p mmi-studio-cli --offline

# 3. Generate SHA-256 release checksum
shasum -a 256 target/release/mmi-studio-cli > docs/deployment/RELEASE_CHECKSUMS.sha256

# 4. Verify checksum manifest
shasum -a 256 -c docs/deployment/RELEASE_CHECKSUMS.sha256
```

---

## 2. Software Bill of Materials (SBOM)

The repository provides standardized machine-readable SBOM files:
- **SPDX 2.3 JSON**: [`docs/deployment/sbom-spdx.json`](../deployment/sbom-spdx.json)
- **CycloneDX 1.5 JSON**: [`docs/deployment/sbom-cyclonedx.json`](../deployment/sbom-cyclonedx.json)

These manifests document all direct and transitive dependencies, licenses, and cryptographic integrity hashes.

---

## 3. Release Artifact Checklist

Before publishing a tagged release:
- [ ] Version updated in `Cargo.toml` and `package.json`.
- [ ] `CHANGELOG.md` updated with release notes.
- [ ] `docs/deployment/RELEASE_CHECKSUMS.sha256` generated and committed.
- [ ] `./scripts/verify-workstation.sh` passes 100% offline.
