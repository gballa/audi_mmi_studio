# How-To: Inspect an Unknown Firmware Binary

This guide walks through the exact steps required to analyze an unidentified binary file from an Audi MMI 3G update package.

---

## Goal
Extract format metadata, detect compression or encryption, find embedded containers, and view raw byte structures.

## Prerequisites
- Compiled `mmi-studio-cli` binary (`target/release/mmi-studio-cli`).
- Target binary file path (e.g. `originals/MU9411/HBNavDB/database.db`).

---

## Procedure

### 1. Detect Basic Format & Hashes
Run `inspect` to check for known magic headers and generate cryptographic hashes:
```bash
./target/release/mmi-studio-cli inspect originals/MU9411/HBNavDB/database.db
```
*Expected Result*: Output displays file size, detected format (e.g., `HBNavDB (.db)`), BLAKE3 hash, and whether digital signatures are present.

### 2. Profile Entropy to Identify Compression / Encryption
Run `entropy` with a 512-byte sliding analysis window:
```bash
./target/release/mmi-studio-cli entropy originals/MU9411/HBNavDB/database.db --window 512
```
*Interpretation*:
- `Entropy < 3.5`: Plaintext headers, sparse tables, zero-padding.
- `Entropy 3.5 - 6.8`: Structured records, code instructions, uncompressed index structures.
- `Entropy 6.9 - 7.9`: Compressed streams (zlib, deflate).
- `Entropy > 7.95`: Encrypted payload or high-density random noise.

### 3. Scan for Embedded Signatures
Run `carve` to scan for inner filesystems and containers:
```bash
./target/release/mmi-studio-cli carve originals/MU9411/HBNavDB/database.db
```
*Expected Result*: Lists any embedded QNX IFS, EFS/F3S, `.precomp`, or archive headers discovered inside the binary with exact byte offsets.

### 4. Render Virtualized Hex View
Render the first 256 bytes of the container header:
```bash
./target/release/mmi-studio-cli hexdump originals/MU9411/HBNavDB/database.db --offset 0 --length 256
```
*Expected Result*: Formatted hex dump showing offsets, hex bytes, and printable ASCII representation.
