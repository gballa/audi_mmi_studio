# Getting Started — First Run & Workstation Verification

This document guides you through your first complete verification pass to confirm that **Audi MMI Studio** is operational, all evidence tags conform to standards, and the safety gates are active.

---

## 1. Running the Automated Verification Gate

The workstation includes an all-in-one verification script:

```bash
./scripts/verify-workstation.sh
```

This script executes five discrete stages strictly offline:
1. **Evidence Tagging Discipline**: Scans all architectural documents with `scripts/check-evidence-tags.py` to ensure all normative claims are traceable.
2. **Recipe Validation**: Validates all pre-built theme recipes in `recipes/` against `mmi_recipe`.
3. **Offline Compilation**: Executes `cargo check --workspace --offline`.
4. **Automated Test Suite**: Runs all 43+ unit, integration, and E2E pipeline tests.
5. **Release Binary Verification**: Confirms that `target/release/mmi-studio-cli` is present and executable.

Expected output banner:
```text
========================================================
 VERIFICATION COMPLETE — ALL GATES PASSED (100% OFFLINE)
 Status: BUILD READY — DEPLOYMENT NOT VERIFIED
========================================================
```

---

## 2. Understanding Automotive Safety Statuses

Audi MMI Studio enforces strict safety statuses defined in specification §14.9:

| Status Verdict | Definition | Allowed Actions |
| :--- | :--- | :--- |
| `BUILD READY — DEPLOYMENT NOT VERIFIED` | Software passed all unit tests and determinism rebuild checks; not yet tested on real vehicle hardware. | Analysis, simulation, packaging |
| `SIMULATED — NOT A GUARANTEE` | Pre-flight flashing simulation succeeded in software; hardware execution may encounter electrical or timing differences. | Verification only |
| `PROTECTED / OUT OF SCOPE — DOCUMENT ONLY` | Binary contains digital signatures or proprietary activation routines. | Read-only analysis |
| `HIGH RISK — NO VERIFIED RECOVERY PATH` | Flash component lacks a documented bootloader recovery fallback. | Locked from staging |

> [!CAUTION]
> Under no circumstances does the workstation claim an update is *"SAFE TO INSTALL"*. This phrase is strictly prohibited to prevent unwarranted operator assumptions.

---

## 3. First Successful Operation Checklist

After completing the first run:
- [x] CLI binary compiled and located at `target/release/mmi-studio-cli`.
- [x] Full workspace tests pass with exit code 0.
- [x] Evidence tagging script reports 0 violations.
- [x] Output directories are ready for staging.

You are now ready to use the workstation. Continue to the [User Guide Overview](../user-guide/overview.md).
