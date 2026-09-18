# Architecture — Runtime Environment & Determinism

This document describes the runtime execution model, memory isolation boundaries, concurrency, and deterministic guarantees in **Audi MMI Studio**.

---

## 1. Execution Model & Concurrency

- **Synchronous Determinism**: Binary packaging, container sorting, and cryptographic hash computations execute synchronously with fixed iteration orders.
- **Multithreaded Validation**: The 6-tier validation suite parallelizes format verification across CPU cores using thread pools while isolating file handles to prevent race conditions.
- **IPC Worker Isolation**: In `mmi-studio-desktop`, all IPC operations execute in worker threads on the native side, preventing webview UI thread freezing during high-throughput entropy calculations.

---

## 2. Memory Isolation & Sandboxing

The platform protects the workstation from malformed or hostile binary containers:
- **Zero Raw Pointers Across Formats**: All decoders under `mmi-formats` operate exclusively on safe byte slices (`&[u8]`) with bounds-checked access.
- **Plugin Memory Limits**: Third-party plugins loaded by `mmi-plugin` run within strict memory-capped execution sandboxes (default maximum: 64 MB heap allocation). Exceeding this limit immediately aborts the adapter task without destabilizing the host application.
- **Content-Addressed Storage Caching**: Raw data chunks are pinned to immutable files in `.mmistudio/cas/`, preventing unbounded RAM consumption when inspecting multi-gigabyte disk images.

---

## 3. Deterministic Packaging Guarantees

To ensure that candidate rebuilds are verifiable bit-for-bit:
1. **Lexicographical Directory Traversal**: `StageNormalizer` sorts all file paths lexicographically before archive generation.
2. **Normalized Metadata**: Timestamps, file owner IDs, and permission bits are normalized to standard defaults (`0755` for executables, `0644` for data files, epoch timestamp `0`).
3. **Reproducible Compression**: zlib compression routines employ fixed compression levels (`level = 6`, fixed window size) to ensure byte-exact reproducibility across platforms.
