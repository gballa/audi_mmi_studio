# Development — Debugging & Diagnostics

This document provides debugging techniques for investigating Rust panics, format decoding errors, and sandbox execution failures.

---

## 1. Enabling Verbose Tracing

Audi MMI Studio uses the `tracing` framework:

```bash
# Enable INFO logs
RUST_LOG=info ./target/release/mmi-studio-cli inspect <file>

# Enable DEBUG logs for all workspace crates
RUST_LOG=mmi=debug ./target/release/mmi-studio-cli validate <dir>

# Enable full TRACE output for deep byte-level debugging
RUST_LOG=trace ./target/release/mmi-studio-cli carve <file>
```

---

## 2. Inspecting Panics & Backtraces

To obtain a full stack backtrace when debugging a panic:

```bash
RUST_BACKTRACE=1 cargo test -p mmi-formats --offline
```

---

## 3. Investigating Format Decoder Failures

When a decoder in `mmi-formats` rejects an input file:
1. **Verify Header Magic**:
   Use `mmi-studio-cli hexdump <file> --length 32` to inspect the first 32 bytes and check for expected magic bytes.
2. **Check File Length & Truncation**:
   Compare the file size on disk with the size recorded in header length fields.
3. **Inspect Entropy**:
   Use `mmi-studio-cli entropy <file> --window 256` to determine if a header is followed by an unexpected compressed stream or encrypted table.

---

## 4. Investigating Sandbox Memory Aborts

If a third-party plugin terminates unexpectedly:
1. Run with `RUST_LOG=mmi_plugin=trace`.
2. Check if the plugin exceeded its allocated `memory_limit_mb` (default 64 MB).
3. Increase `memory_limit_mb` in `plugin.json` if required and re-verify.
