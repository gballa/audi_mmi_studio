#!/usr/bin/env bash
set -euo pipefail

# Audi MMI Studio — Unified Workstation Verification Gate
# Runs 100% offline verification across documentation, linting, tests, recipes, and release binaries.

WORKSPACE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WORKSPACE_DIR"

echo "========================================================"
echo " Audi MMI Studio — Workstation Verification Gate"
echo "========================================================"
echo "Timestamp: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo "Working directory: $WORKSPACE_DIR"
echo ""

# Ensure Cargo environment is loaded
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# 1. Evidence Tagging & Documentation Links Discipline
echo "[1/5] Checking evidence tags and documentation links..."
/usr/bin/python3 scripts/check-evidence-tags.py
/usr/bin/python3 scripts/check-docs-links.py
echo "  -> Evidence tags and internal documentation links conform 100%."
echo ""

# 2. Recipe Schema Validation
echo "[2/5] Validating declarative theme recipes in recipes/..."
for recipe in recipes/*.json; do
    if [ -f "$recipe" ]; then
        /usr/bin/python3 -c "import json, sys; d=json.load(open('$recipe')); assert 'api_version' in d and 'operations' in d; print('  -> Validated:', '$recipe', f'({len(d[\"operations\"])} ops)')"
    fi
done
echo ""

# 3. Cargo Check Offline
echo "[3/5] Compiling and linting workspace crates (--offline)..."
cargo check --offline --workspace --all-targets
echo "  -> Compilation clean."
echo ""

# 4. Full Workspace Offline Test Suite
echo "[4/5] Executing full offline test suite across all 14 crates..."
cargo test --offline --workspace
echo "  -> All workspace tests passed."
echo ""

# 5. Production Release Binary Verification
echo "[5/5] Verifying production release binary..."
if [ -f "target/release/mmi-studio-cli" ]; then
    CLI_VER=$(target/release/mmi-studio-cli --version)
    echo "  -> Standalone release binary verified: $CLI_VER"
else
    echo "  -> Building standalone release binary..."
    cargo build --release --offline -p mmi-studio-cli
    CLI_VER=$(target/release/mmi-studio-cli --version)
    echo "  -> Standalone release binary built: $CLI_VER"
fi
echo ""

echo "========================================================"
echo " VERIFICATION COMPLETE — ALL GATES PASSED (100% OFFLINE)"
echo " Status: BUILD READY — DEPLOYMENT NOT VERIFIED"
echo "========================================================"
