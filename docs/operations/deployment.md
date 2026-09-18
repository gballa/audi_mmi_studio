# Operations — In-Vehicle Hardware Deployment

This document provides procedures for deploying firmware and theme updates to an Audi MMI 3G High / Plus head unit via the Red Engineering Menu (REM).

---

## 1. Safety & Preparation

> [!CAUTION]
> **Battery Voltage Requirement**:
> Flashing automotive head units on internal vehicle battery alone carries an extreme risk of bricking due to low-voltage shutdowns. Connect a professional battery maintainer (minimum 13.5V continuous, 25A–40A rating) to the vehicle jump posts under the hood before beginning.

### Equipment Required
- Verified SD card formatted as FAT32 with 32 KiB cluster geometry.
- Staged deployment package created with `mmi-studio-cli build-media`.
- Automotive power supply / battery maintainer.

---

## 2. Flashing Procedure via Red Engineering Menu (REM)

1. **Power On the Vehicle**:
   Switch ignition ON. Do not start the engine. Ensure lights, air conditioning, and seat heaters are turned OFF.
2. **Open the Red Engineering Menu**:
   Press and hold `[CAR] + [BACK]` simultaneously for 5 seconds until the screen switches to the red engineering interface.
3. **Insert Media**:
   Insert the prepared SD card into **SD Slot 1** (left slot).
4. **Initiate Update**:
   - Select **Update** in the REM menu.
   - Choose **SD 1** as the source.
   - Select **Standard** update mode (or **User-Defined** if staging specific custom skin modules).
   - Review module checklist.
   - Select **Start Update**.
5. **Update Execution**:
   The system will reboot into QNX flashing mode and display progress bars for each updated component. Do not touch any buttons or disconnect power during this phase (typically 5–15 minutes).
6. **Completion**:
   Once all components report `OK`, select **Continue** and execute the **Documentation Cancel** step to reboot the system into normal operation.
