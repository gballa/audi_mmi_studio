# How-To: Generate Emergency Stock Recovery Bundles

This guide explains how to produce a self-contained emergency recovery package from the genuine stock firmware baseline to restore an Audi MMI 3G head unit in the event of an update failure.

---

## Goal
Create an emergency rollback bundle containing pristine stock binaries and standalone POSIX recovery scripts for direct QNX IPL / UART console execution.

## Prerequisites
- Pristine firmware baseline in `originals/` (e.g. `HN+R_EU_AU_K0942_4_[8R0906961FB]`).
- Compiled `mmi-studio-cli`.

---

## Procedure

### Step 1: Package Stock Recovery Bundle
Run the `stock-recovery` command:
```bash
./target/release/mmi-studio-cli stock-recovery \
  --baseline-train "HN+R_EU_AU_K0942_4_[8R0906961FB]" \
  --output output/stock_recovery_bundle
```

### Step 2: Inspect Output Bundle Artifacts
Check the contents of `output/stock_recovery_bundle/`:
- `manifest.json`: Cryptographic index of all stock modules and BLAKE3/SHA-256 hashes.
- `emergency_rollback.sh`: Standalone POSIX shell script runnable via QNX UART terminal.
- `STOCK_SHA256SUMS`: Verification hashes.
- Pristine copies of all core system binaries.

### Step 3: Flash to Emergency Recovery Media
Copy the entire contents of `output/stock_recovery_bundle/` to a dedicated, high-quality FAT32 SD card. Label this card clearly as `MMI_EMERGENCY_RECOVERY` and keep it on hand during vehicle flashing operations.

---

## Emergency Execution via QNX UART

In the event of an interrupted update or boot loop:
1. Connect a 3.3V USB-to-UART serial adapter to the MMI Quadlock diagnostic port at 115200 baud (8N1).
2. Insert the Emergency Recovery SD card into SD Slot 1.
3. Access the QNX root shell prompt (`#`).
4. Execute the rollback script:
   ```sh
   cd /fs/sda0
   sh emergency_rollback.sh
   ```
5. Trigger a system sync and reboot:
   ```sh
   sync && shutdown -S
   ```
