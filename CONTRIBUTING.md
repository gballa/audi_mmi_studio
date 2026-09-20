# Contributing to Audi MMI Studio

Thank you for your interest in contributing to **Audi MMI Studio**. This project is an offline-first reverse-engineering workstation and asset toolchain for automotive infotainment systems.

To ensure safety, determinism, and architectural integrity, all contributions must strictly adhere to the guidelines established in this document.

---

## Core Principles & Safety Rules

1. **Strict Immutability of `originals/`**:
   The `originals/` reference corpus is strictly read-only. No script, test, or code change may modify, write to, or delete files within `originals/`. All operations must use the `SourceStore` abstraction.

2. **Offline-First Guarantee**:
   All builds, tests, scripts, and CI checks must execute with zero external network connectivity. Every cargo command must pass `--offline`.

3. **Signed Payload Protection**:
   Binary payloads containing cryptographic digital signatures (`.pkg.sig`, `.dat.sig`, QNX IFS/IPL bootloaders) are analysis-only. Modifying, repackaging, or staging signed binaries is strictly prohibited by the Identity-Rebuild Gate (`mmi-rebuild`).

4. **Safety Status Vocabulary**:
   Automotive readiness verdicts must strictly employ the approved safety vocabulary (§14.9):
   - `VERIFIED`
   - `SUPPORTED`
   - `PARTIALLY SUPPORTED`
   - `EXPERIMENTAL`
   - `UNSUPPORTED`
   - `UNKNOWN`
   - `RESEARCH REQUIRED`
   - `BUILD READY — DEPLOYMENT NOT VERIFIED`
   - `SIMULATED — NOT A GUARANTEE`
   - `PROTECTED / OUT OF SCOPE — DOCUMENT ONLY`
   - `HIGH RISK — NO VERIFIED RECOVERY PATH`

   > [!CAUTION]
   > The phrase `"SAFE TO INSTALL"` is **permanently forbidden** across all documentation, UI strings, logs, and code.

---

## Development Setup

### Prerequisites

- **Rust**: 1.80+ (stable toolchain)
- **Node.js**: 18+ and `npm` (for the desktop UI)
- **Python**: 3.9+ (for documentation validation scripts)

### Getting the Code

```bash
# Clone the repository
git clone <repo-url> audi_mmi_studio
cd audi_mmi_studio

# Verify workstation environment and test suite offline
./scripts/verify-workstation.sh
```

---

## Coding Standards

### Rust Workstation Crates

- **Formatting**: Format all code with `cargo fmt`.
- **Linter**: Ensure clean `cargo clippy --workspace --all-targets --offline -- -D warnings`.
- **Error Handling**: Use `thiserror` for crate-internal typed errors and ensure clear diagnostic context. Avoid unwraps in library code; handle errors gracefully.
- **Dependencies**: Keep workspace dependencies minimal. Add dependencies only to the root `Cargo.toml` `[workspace.dependencies]` table before linking into individual crates.

### Desktop Application (`apps/mmi-studio-desktop`)

- **Architecture**: Keep binary processing in native Rust (`src-tauri`). The React 18 / TypeScript frontend communicates exclusively via strongly typed IPC commands.
- **Strict Local Origin**: Frontend assets must run locally under `tauri://localhost`. Zero remote scripts or CDN assets.

---

## Testing & Verification Workflow

Before submitting a Pull Request, run the full verification gate:

```bash
# 1. Run full offline workspace tests
cargo test --workspace --offline

# 2. Check documentation evidence tagging
/usr/bin/python3 scripts/check-evidence-tags.py

# 3. Check internal documentation links
/usr/bin/python3 scripts/check-docs-links.py

# 4. Run full workstation verification gate
./scripts/verify-workstation.sh
```

All five verification stages must exit with code 0.

---

## Pull Request Checklist

When submitting a PR, ensure:

- [ ] New functionality is covered by unit and/or integration tests.
- [ ] No network calls are introduced in workspace crates.
- [ ] `originals/` remains unmodified.
- [ ] Documentation is updated under `docs/` reflecting any new features or API changes.
- [ ] `CHANGELOG.md` is updated under the `[Unreleased]` section.
- [ ] `./scripts/verify-workstation.sh` passes 100% offline.
