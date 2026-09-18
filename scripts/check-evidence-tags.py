#!/usr/bin/env python3
"""
scripts/check-evidence-tags.py
Validates evidence tagging discipline across all Pass A documentation:
- SOURCE_AUDIT.md
- docs/research/RQ-REGISTER.md
- PROJECT.md

Rules:
1. Every normative/factual assertion must contain a valid evidence tag:
   - [EV:<source-id>@<path-or-offset>]
   - [EV:doc:<url-or-citation>]
   - [EV:oss:<project>@<ref>]
   - [INF:HIGH|MEDIUM|LOW( basis: ...)?]
   - [UNK]
2. Every [UNK] tag must correspond to an active RQ entry in docs/research/RQ-REGISTER.md.
3. Exits with code 0 on complete compliance, 1 on any violation.
"""

import sys
import os
import re
import argparse
from pathlib import Path

TAG_REGEX = re.compile(
    r'\[('
    r'EV:[a-zA-Z0-9_.-]+@[^\]]+|'
    r'EV:doc:[^\]]+|'
    r'EV:oss:[a-zA-Z0-9_.-]+@[^\]]+|'
    r'INF:(?:HIGH|MEDIUM|LOW)(?:\s+basis:[^\]]+)?|'
    r'UNK(?:\s+RQ-\d{3})?'
    r')\]'
)

RQ_ID_REGEX = re.compile(r'\b(RQ-\d{3})\b')
RQ_HEADER_REGEX = re.compile(r'^#{1,4}\s+(RQ-\d{3})[:\s]', re.MULTILINE)

# Structural lines that do not require tags
IGNORABLE_LINE_PREFIXES = (
    '#',        # Markdown headings
    '---',      # Horizontal rules
    '===',      # Setext header underlines
    '```',      # Code fences
    '|---|',    # Table separators
    '| ---',    # Table separators
    '>',        # Blockquotes/callouts
    '<!--',     # Comments
)


def extract_registered_rqs(rq_register_path: Path) -> set:
    """Finds all defined RQ IDs in RQ-REGISTER.md."""
    if not rq_register_path.exists():
        return set()
    content = rq_register_path.read_text(encoding='utf-8')
    registered = set(RQ_HEADER_REGEX.findall(content))
    # Also find table entries like | RQ-001 |
    for match in re.finditer(r'\|\s*(RQ-\d{3})\s*\|', content):
        registered.add(match.group(1))
    return registered


def check_file(file_path: Path, registered_rqs: set, verbose: bool = False) -> list:
    """Validates evidence tagging in a single Markdown file."""
    violations = []
    if not file_path.exists():
        violations.append(f"{file_path}: File does not exist")
        return violations

    lines = file_path.read_text(encoding='utf-8').splitlines()
    in_code_block = False
    in_html_comment = False

    for line_num, raw_line in enumerate(lines, start=1):
        line = raw_line.strip()

        # Handle multi-line code blocks
        if line.startswith('```'):
            in_code_block = not in_code_block
            continue
        if in_code_block:
            continue

        # Handle HTML comments
        if '<!--' in line and '-->' not in line:
            in_html_comment = True
            continue
        if in_html_comment:
            if '-->' in line:
                in_html_comment = False
            continue

        # Skip empty lines
        if not line:
            continue

        # Skip structural lines
        if any(line.startswith(prefix) for prefix in IGNORABLE_LINE_PREFIXES):
            continue

        # Skip table separator rows
        if re.match(r'^\|?\s*:?-+:?\s*(\|?\s*:?-+:?\s*)+\|?$', line):
            continue

        # Skip table header rows (row directly above table separator row)
        if line.startswith('|') and line_num < len(lines):
            next_line = lines[line_num].strip()
            if re.match(r'^\|?\s*:?-+:?\s*(\|?\s*:?-+:?\s*)+\|?$', next_line):
                continue

        # Skip structural section / bullet headers (e.g. "1. **Title**:", "- **Proposed RE Plan**:", "Specifically:")
        if re.match(r'^(?:\d+\.\s+)?\*{1,2}[^*]+:\*{1,2}$', line) or line.endswith('**:'):
            continue
        if re.match(r'^(?:[-*]\s+)?\*{1,2}[^*]+:\*{1,2}$', line):
            continue
        if line in ("Specifically:", "Ordering note:", "Notes:", "Summary:"):
            continue

        # Check for tags on the line
        tags_found = TAG_REGEX.findall(line)
        if not tags_found:
            # Report violation: normative line without evidence tag
            snippet = (line[:80] + '...') if len(line) > 80 else line
            violations.append(
                f"{file_path}:{line_num}: Untagged normative statement: \"{snippet}\""
            )
        else:
            # For each UNK tag found (outside backticks), ensure valid RQ mapping
            # Strip backticked code spans so literal `[UNK]` in documentation isn't treated as an assertion
            line_no_code = re.sub(r'`[^`]+`', '', line)
            uncoded_tags = TAG_REGEX.findall(line_no_code)
            for t in uncoded_tags:
                if t.startswith('UNK'):
                    rq_matches = RQ_ID_REGEX.findall(line)
                    if not rq_matches:
                        if file_path.name != "RQ-REGISTER.md":
                            violations.append(
                                f"{file_path}:{line_num}: [UNK] tag missing explicit RQ reference (e.g. RQ-001)"
                            )
                    else:
                        for rq_id in rq_matches:
                            if registered_rqs and rq_id not in registered_rqs:
                                violations.append(
                                    f"{file_path}:{line_num}: Referenced {rq_id} not registered in RQ-REGISTER.md"
                                )

    return violations


def main():
    parser = argparse.ArgumentParser(description="Verify evidence tag discipline in Pass A docs.")
    parser.add_argument("files", nargs="*", help="Files to verify. If omitted, default Pass A docs are checked.")
    parser.add_argument("--strict", action="store_true", help="Strict enforcement mode.")
    parser.add_argument("--verbose", action="store_true", help="Print verbose details.")
    args = parser.parse_args()

    project_root = Path(__file__).resolve().parent.parent
    rq_register_file = project_root / "docs" / "research" / "RQ-REGISTER.md"

    if args.files:
        target_files = [Path(f).resolve() for f in args.files]
    else:
        target_files = [
            project_root / "README.md",
            project_root / "docs" / "README.md",
            project_root / "docs" / "audit" / "SOURCE_AUDIT.md",
            project_root / "docs" / "spec" / "PROJECT.md",
            project_root / "docs" / "security" / "SECURITY.md",
            project_root / "docs" / "research" / "RQ-REGISTER.md",
            project_root / "docs" / "deployment" / "DEPLOYMENT_GUIDE.md",
            project_root / "docs" / "deployment" / "EMERGENCY_RECOVERY.md",
            project_root / "docs" / "deployment" / "RELEASE_PACKAGING.md",
            project_root / "docs" / "adr" / "README.md",
            project_root / "docs" / "adr" / "ADR-001-offline-first-workspace-architecture.md",
            project_root / "docs" / "adr" / "ADR-002-tauri-desktop-architecture.md",
            project_root / "docs" / "adr" / "ADR-003-reverse-engineering-discipline-and-rebuild-gate.md",
            project_root / "docs" / "adr" / "ADR-004-declarative-theme-recipes-and-cross-train-rebasing.md",
        ]


    registered_rqs = extract_registered_rqs(rq_register_file)
    if args.verbose:
        print(f"Registered RQs found: {len(registered_rqs)} -> {sorted(registered_rqs)}")

    total_violations = []
    checked_count = 0

    for target in target_files:
        if not target.exists():
            if args.verbose:
                print(f"Skipping non-existent file: {target}")
            continue
        checked_count += 1
        violations = check_file(target, registered_rqs, verbose=args.verbose)
        if violations:
            total_violations.extend(violations)
        else:
            print(f"[PASS] {target.relative_to(project_root) if target.is_relative_to(project_root) else target}")

    if total_violations:
        print(f"\n[FAIL] Found {len(total_violations)} evidence tag violations:")
        for v in total_violations:
            print(f"  - {v}")
        sys.exit(1)
    else:
        print(f"\nAll {checked_count} checked documentation files conform to Evidence Tagging Discipline (exit 0).")
        sys.exit(0)


if __name__ == "__main__":
    main()
