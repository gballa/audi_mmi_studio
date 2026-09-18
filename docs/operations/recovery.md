# Operations — Emergency Recovery & Rollback

This document provides procedures for recovering an Audi MMI 3G head unit from update failures, boot loops, and corrupt filesystems.

---

## 1. Level 1 Recovery: Three-Button Hard Reset

If the MMI interface becomes unresponsive or hangs on the splash screen:
1. Press and hold the three-button reboot combination simultaneously:
   - `[SETUP]` (or `[MENU]` on newer controls)
   - `[CENTRAL ROTARY CONTROLLER]`
   - `[UPPER-RIGHT SOFTKEY]`
2. Hold for 3 seconds until the screen switches off.
3. Release all buttons. The system will perform a full hardware reboot.

---

## 2. Level 2 Recovery: Emergency Red Menu Stock Rollback

If the head unit boots into an update error loop:
1. Insert the **Emergency Recovery SD Card** (generated via `mmi-studio-cli stock-recovery`) into **SD Slot 1**.
2. Force the head unit into the Red Engineering Menu (`[CAR] + [BACK]` for 5 seconds).
3. Select **Standard Update** from **SD 1**.
4. Allow the system to reinstall the verified stock firmware baseline.

---

## 3. Level 3 Recovery: Low-Level QNX UART Shell Rollback

If the head unit cannot launch the GUI or enter the Red Engineering Menu (severe boot loop):
1. Connect a 3.3V USB-to-UART adapter to Quadlock Pins 5, 7, and 11 at 115200 baud.
2. Insert the Emergency Recovery SD card into SD Slot 1.
3. Power cycle the head unit and interrupt bootloader execution to reach the QNX shell:
   ```text
   Press any key to stop autoboot...
   #
   ```
4. Mount the SD card and execute the standalone rollback script:
   ```sh
   mount -t dos /dev/hd1t77 /fs/sda0
   cd /fs/sda0
   sh emergency_rollback.sh
   ```
5. Confirm that all original system slices are restored, sync filesystem caches, and reboot:
   ```sh
   sync
   shutdown -S
   ```
