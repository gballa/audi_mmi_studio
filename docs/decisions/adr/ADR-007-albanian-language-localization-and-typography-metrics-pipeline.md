# ADR-007: Albanian Language Localization and Typography Metrics Pipeline

## Status
ACCEPTED

## Date
2026-09-19

## Context
Factory Audi MMI 3G/3G+ firmware supports major European languages (German, English, French, Italian, Spanish, etc.) but completely lacks Albanian language support (`sq_AL`).
Automotive infotainment localization imposes strict constraints:
- **Harman ANS Binary Catalog**: Text resources are stored in proprietary Harman ANS binary formats (`.ans`) with string key hashes and index offsets.
- **Embedded Display Geometry**: The 800x480 resolution instrument and console displays have fixed pixel bounding boxes for buttons, navigation prompts, and status bars.
- **Typography Overflow**: Albanian strings often have differing character counts and diacritics (`ë`, `ç`) compared to German or English. A string exceeding bounding box constraints causes visual truncation, text wrapping artifacts, or memory overflows in the HMI rendering thread.

## Decision
Establish an integrated Albanian localization and typography validation pipeline:
1. **Albanian String Catalog (`sq_AL.ans`)**:
   - Translate all core system domains: Navigation, Radio, Media, Telephone, Car Setup, and Info.
   - Encode strings using standard UTF-8 and ISO-8859-16 character tables compatible with MMI TrueType fonts.
   - Package catalogs into native Harman ANS binary files (`strings/sq_AL.ans`) placed in `efs-system.efs`.
2. **Typography Overflow Engine (`mmi-assets` / `mmi-studio-cli`)**:
   - Compute bounding box dimensions (width, height, ascent, descent) using genuine Audi MMI TrueType font metrics (`AudiType-Extended.ttf`).
   - Flag strings exceeding UI element bounding boxes prior to packaging.
3. **Studio GUI Visualization (`TypographyStudio.tsx`)**:
   - Provide interactive string inspection, search, translation editing, and real-time bounding box overflow simulation.

## Alternatives Considered

### Direct In-Place Hex Replacement of Strings in `lsd.jxe`
- Pros: No need to understand the ANS catalog structure.
- Cons: Strings must match the exact byte length of original strings (requiring padding with spaces or truncation). Severely degrades translation quality.
- Rejected: Poor readability and high crash potential.

### External Runtime Overlay
- Pros: Leaves flash untouched.
- Cons: Requires secondary hardware (e.g. Raspberry Pi or CAN interceptor).
- Rejected: Violates native head-unit execution requirement.

## Consequences
- Full native Albanian interface without visual truncation or clipping.
- Automated font metric verification prevents text overflow crashes at compile time.
- Extensible architecture enables adding further regional languages using the same pipeline.
