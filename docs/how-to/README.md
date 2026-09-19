# How-To Guides — Master Index

The **How-To Library** provides concise, procedural guides for accomplishing specific engineering and operational tasks in **Audi MMI Studio**. Each guide answers: *"I need to accomplish X. What exactly do I do?"*

---

## Reverse Engineering & Analysis

- [**Inspect an Unknown Binary**](inspect-unknown-binary.md): Step-by-step workflow to detect format, compute Shannon entropy, scan for signatures, and dump byte ranges.
- [**Develop a Format Plugin**](develop-format-plugin.md): How to create, package, and verify an untrusted third-party format adapter using the versioned ABI.

---

## Visual Assets & Theming

- [**Decode and Replace UI Assets**](decode-and-replace-assets.md): How to export proprietary `.precomp` bitmaps to PNG, modify them, conform constraints, and verify the rebuild gate.
- [**Create and Rebase a Theme Recipe**](create-and-rebase-theme-recipe.md): How to author a declarative JSON theme recipe, apply it to a staging workspace, and evaluate cross-train drift.

---

## Rebuild, Verification & Packaging

- [**Produce Custom Firmware with Albanian Language & 2026 Maps**](produce-albanian-and-2026-maps-firmware.md): Complete end-to-end tutorial for configuring RS themes, Albanian translations (`sq_AL`), 2026 cartography, diagnostic tools, and flashing SD media.
- [**Complete In-Car SD Card Upgrade Guide**](in-car-sd-update-guide.md): Detailed step-by-step procedure for preparing media, in-car SWDL flashing, navigation unblocking, and post-update diagnostics.
- [**Rebuild and Validate Firmware**](rebuild-and-validate-firmware.md): How to deterministically repackage staged files and execute the 6-tier (`L0`–`L5`) automotive validation suite.
- [**Prepare SD Deployment Media**](prepare-sd-deployment-media.md): How to partition, format with 32 KiB cluster geometry, and package update files onto FAT32 SD media.
- [**Simulate QNX Head-Unit Update**](simulate-qnx-update.md): How to run the pre-flight flashing simulation to verify dependency graphs and partition limits.

---

## Recovery & Emergency Operations

- [**Generate Emergency Stock Recovery Bundle**](emergency-stock-recovery.md): How to identify pristine stock baseline packages and produce self-contained recovery media with UART restoration scripts.
