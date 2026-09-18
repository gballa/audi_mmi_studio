# Development — Workflow & Coding Standards

This document explains the development lifecycle, branch workflow, coding conventions, and documentation standards for **Audi MMI Studio**.

---

## 1. Branching & Commit Discipline

- **Main Branch**: `main` is always build-ready and passes all 5 stages of `./scripts/verify-workstation.sh`.
- **Feature Branches**: Branch from `main` using descriptive names:
  - `feat/format-<format-name>`: New format decoders.
  - `fix/rebuild-<issue>`: Rebuild gate or verification fixes.
  - `docs/<topic>`: Documentation enhancements.
- **Commit Messages**: Follow Conventional Commits:
  - `feat(formats): add decoder for QNX 6 EFS directory entries`
  - `fix(rebuild): prevent signed asset replacement in stage normalizer`
  - `docs(reference): document all 22 CLI subcommands`

---

## 2. Coding Standards

### Rust Code
- **Format**: Run `cargo fmt --all` before every commit.
- **Lints**: All code must pass `cargo clippy --workspace --offline -- -D warnings`.
- **Error Handling**:
  - Never use `.unwrap()` or `.expect()` in library crates (`crates/*`).
  - Use `thiserror` to define domain-specific errors (e.g. `CoreError`, `FormatError`).
- **Memory Safety**:
  - Unsafe code is strictly forbidden unless absolutely required for FFI.
  - Slices (`&[u8]`) must be used for binary buffer inspection.

### Documentation Standards
- **Evidence Tagging Discipline**: Every normative statement in core architectural documentation must carry a valid evidence tag (`[EV:...]`, `[INF:...]`, `[UNK]`) verified by `scripts/check-evidence-tags.py`.
- **Safety Statuses**: Use only approved status vocabulary (`BUILD READY — DEPLOYMENT NOT VERIFIED`, `SIMULATED — NOT A GUARANTEE`, etc.). The phrase `"SAFE TO INSTALL"` is permanently banned.

---

## 3. Pull Request Validation Workflow

Before pushing code:
```bash
# 1. Format code
cargo fmt --all

# 2. Run clippy
cargo clippy --workspace --offline

# 3. Run workspace tests
cargo test --workspace --offline

# 4. Check evidence tags
/usr/bin/python3 scripts/check-evidence-tags.py

# 5. Check docs links
/usr/bin/python3 scripts/check-docs-links.py

# 6. Run full workstation verification gate
./scripts/verify-workstation.sh
```
