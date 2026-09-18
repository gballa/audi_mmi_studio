# Development — Testing Strategy

This document outlines the testing tiers, execution commands, integration tests, and End-to-End (E2E) theming pipeline tests in **Audi MMI Studio**.

---

## 1. Testing Tiers

The workspace includes 43+ automated tests organized across three testing levels:

```text
┌─────────────────────────────────────────────────────────────┐
│ Level 3: End-to-End System Tests (E2E Theming Pipeline)    │
│ - Full cycle: extract -> recipe apply -> rebuild -> media   │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│ Level 2: Integration & CLI Tests (tests/cli_tests.rs)       │
│ - All 22 CLI subcommands, IPC bridges, sandbox isolation    │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│ Level 1: Crate Unit Tests (crates/*/src/)                   │
│ - Decoders, hash computations, entropy, conformers          │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Test Execution Commands

All tests must execute 100% offline:

```bash
# Run all workspace unit and integration tests
cargo test --workspace --offline

# Run only CLI integration tests
cargo test -p mmi-studio-cli --test cli_tests --offline

# Run the full End-to-End theming pipeline test
cargo test -p mmi-studio-cli --test e2e_theming_tests --offline

# Run Desktop Tauri IPC tests
cargo test -p mmi-studio-desktop --test desktop_tests --offline
```

---

## 3. End-to-End (E2E) Theming Pipeline Test

Located in `apps/mmi-studio-cli/tests/e2e_theming_tests.rs`:
- Creates an isolated staging environment in a temporary directory.
- Copies genuine sample `.precomp` files from `originals/`.
- Applies the `audi_sport_amber.json` theme recipe.
- Verifies that color swaps are written correctly.
- Evaluates the rebuilt binary with the Identity-Rebuild Gate.
- Builds a simulated FAT32 SD media volume.
- Executes the QNX update simulation and validates the output manifest.

---

## 4. Writing New Tests

When adding features:
- Place unit tests in a `tests` module within `src/` or integration tests in `tests/`.
- Never write tests that require internet connectivity.
- Never write tests that mutate files inside `originals/`. Use `tempfile::tempdir()` for all temporary files.
