#!/usr/bin/env bash
# ==============================================================================
# Script: scripts/validate-antigravity.sh
# Purpose: Validates workspace manifests, character limits, and JSON integrity
# ==============================================================================
set -euo pipefail

EXIT_CODE=0
WORKSPACE_ROOT="$(pwd)"
MAX_RULE_CHARS=12000

echo "=========================================================="
echo "    Auditing Antigravity Configuration & Manifests        "
echo "=========================================================="

# Check File Existence & Character Limits
check_file() {
    local file="$1"
    local max_chars="${2:-0}"
    if [ -f "$file" ]; then
        local count
        count=$(wc -m < "$file")
        if [ "$max_chars" -gt 0 ] && [ "$count" -gt "$max_chars" ]; then
            echo "  [FAIL] $file exceeds platform limit! ($count > $max_chars chars)"
            EXIT_CODE=1
        else
            echo "  [PASS] Found: $file ($count chars)"
        fi
    else
        echo "  [FAIL] Missing file: $file"
        EXIT_CODE=1
    fi
}

# Check Frontmatter Field Presence
check_frontmatter_field() {
    local file="$1"
    local field="$2"
    if [ -f "$file" ]; then
        if grep -q "^${field}:" "$file"; then
            echo "         - Validated frontmatter field: '${field}'"
        else
            echo "  [FAIL] $file missing required frontmatter '${field}:'"
            EXIT_CODE=1
        fi
    fi
}

echo ""
echo "--- 1. Checking Root Configuration & Hooks ---"
check_file "${WORKSPACE_ROOT}/GEMINI.md" "$MAX_RULE_CHARS"
check_file "${WORKSPACE_ROOT}/.agents/hooks.json" 0

echo ""
echo "--- 2. Checking Workspace Rules ---"
for rule in git-workflow testing-standards; do
    target_rule="${WORKSPACE_ROOT}/.agents/rules/${rule}.md"
    check_file "$target_rule" "$MAX_RULE_CHARS"
    check_frontmatter_field "$target_rule" "activation"
done

echo ""
echo "--- 3. Checking Specialized Agent Manifests ---"
AGENTS=(
    "architect"
    "planner"
    "implementer"
    "debugger"
    "test-engineer"
    "code-reviewer"
    "security-reviewer"
    "release-engineer"
)
for agent in "${AGENTS[@]}"; do
    target_agent="${WORKSPACE_ROOT}/.agents/agents/${agent}.md"
    check_file "$target_agent" "$MAX_RULE_CHARS"
    check_frontmatter_field "$target_agent" "name"
    check_frontmatter_field "$target_agent" "description"
    check_frontmatter_field "$target_agent" "tools"
done

echo ""
echo "--- 4. Checking Progressive Disclosure Skills ---"
SKILLS=(
    "repository-analysis"
    "self-verification"
    "code-review"
    "technical-research"
)
for skill in "${SKILLS[@]}"; do
    target_skill="${WORKSPACE_ROOT}/.agents/skills/${skill}/SKILL.md"
    check_file "$target_skill" "$MAX_RULE_CHARS"
    check_frontmatter_field "$target_skill" "name"
    check_frontmatter_field "$target_skill" "description"
done

# Ensure helper scripts have executable bits
HELPER_SCRIPT="${WORKSPACE_ROOT}/.agents/skills/repository-analysis/scripts/analyze_structure.py"
if [ -f "$HELPER_SCRIPT" ]; then
    if [ -x "$HELPER_SCRIPT" ]; then
        echo "  [PASS] Executable bit set on ${HELPER_SCRIPT}"
    else
        echo "  [WARN] Fixing missing executable permission on ${HELPER_SCRIPT}..."
        chmod +x "$HELPER_SCRIPT"
    fi
fi

echo ""
echo "--- 5. Validating JSON Schemas ---"
for json_file in "${WORKSPACE_ROOT}/.agents/hooks.json"; do
    if [ -f "$json_file" ]; then
        if python3 -m json.tool "$json_file" > /dev/null 2>&1; then
            echo "  [PASS] Valid JSON syntax: $json_file"
        else
            echo "  [FAIL] Invalid JSON syntax: $json_file"
            EXIT_CODE=1
        fi
    fi
done

echo ""
echo "=========================================================="
if [ "$EXIT_CODE" -eq 0 ]; then
    echo "  ALL CHECKS PASSED: Antigravity environment is valid!"
    echo "=========================================================="
else
    echo "  AUDIT FAILED: One or more validation checks failed."
    echo "=========================================================="
fi

exit $EXIT_CODE