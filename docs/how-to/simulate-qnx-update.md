# How-To: Simulate QNX Head-Unit Update

This guide explains how to execute the pre-flight QNX head-unit update simulation to detect update failure points before touching vehicle hardware.

---

## Goal
Verify `metainfo2.txt` syntax, module dependency ordering, and partition boundaries in software.

## Prerequisites
- Prepared SD media directory (e.g. `output/sd_deploy`).
- Compiled `mmi-studio-cli`.

---

## Procedure

### Step 1: Run Pre-Flight Simulation
Execute `simulate-update` against your prepared deployment media:
```bash
./target/release/mmi-studio-cli simulate-update output/sd_deploy
```

To output full machine-readable details as JSON:
```bash
./target/release/mmi-studio-cli simulate-update output/sd_deploy --json
```

---

## Simulation Checks Evaluated

The simulator evaluates the exact logic of the QNX `installer` binary:
1. **Release Manifest Check**: Validates `metainfo2.txt` file structure, version tags, and checksum hashes.
2. **Dependency Sequencing**: Ensures prerequisite drivers (e.g. FPGA, DSP) install prior to application modules (`MU9411`).
3. **Partition Capacity**: Compares uncompressed module footprints against target QNX flash slice boundaries.
4. **Signature Status**: Confirms that non-factory keys are not required for any staged components.

### Interpreting Simulation Results
```text
=== QNX MMI 3G Update Pre-Flight Simulator ===
[PASS] metainfo2.txt parsed successfully
[PASS] Module dependency graph is acyclic and valid
[PASS] IFS partition capacity: 14.2 MB / 16.0 MB (88.7%)
[PASS] EFS partition capacity: 48.1 MB / 64.0 MB (75.1%)
Status: SIMULATED — NOT A GUARANTEE
```

> [!NOTE]
> A successful simulation status (`SIMULATED — NOT A GUARANTEE`) proves software correctness, but does not account for physical hardware faults (e.g., loose SD pins or battery voltage drop).
