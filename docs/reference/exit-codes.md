# Reference — Process Exit Codes

This document defines the process exit codes returned by `mmi-studio-cli` and diagnostic scripts.

---

## Exit Code Registry

| Exit Code | Identifier | Meaning | Common Cause |
| :---: | :--- | :--- | :--- |
| **`0`** | `EXIT_SUCCESS` | Operation succeeded; all checks and validations passed. | Normal completion. |
| **`1`** | `EXIT_FAILURE` | General operation error, format parsing error, or verification failure. | Invalid file, checksum mismatch, or failed unit test. |
| **`2`** | `EXIT_USAGE` | Invalid command syntax, missing required argument, or unknown flag. | CLI syntax error (e.g. missing `-o` output path). |
| **`3`** | `EXIT_SIGNED_PAYLOAD_LOCK` | Safety lock violation (§1.4). Attempted modification or rebuild of a signed binary payload. | Attempting to replace or rebuild `.pkg.sig`, `.dat.sig`, or QNX IFS bootloader without private keys. |

---

## Exit Code Handling in Shell Scripts

```bash
#!/bin/bash
./target/release/mmi-studio-cli verify-rebuild "$TARGET_FILE"
EXIT_STATUS=$?

case $EXIT_STATUS in
  0)
    echo "Rebuild gate passed."
    ;;
  3)
    echo "CRITICAL: Binary is signed and locked! Modification forbidden."
    exit 3
    ;;
  *)
    echo "Verification failed with exit code $EXIT_STATUS."
    exit $EXIT_STATUS
    ;;
esac
```
