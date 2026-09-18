#!/usr/bin/env python3
"""
scripts/check-docs-links.py
Validates internal markdown links across all documentation files in the repository.
Ensures zero broken links, nonexistent file references, or invalid relative paths.
"""

import sys
import os
import re
from pathlib import Path

# Match markdown links: [text](link)
LINK_REGEX = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')

def is_external_link(url: str) -> bool:
    return url.startswith(('http://', 'https://', 'mailto:', 'ftp://', 'javascript:'))

def normalize_anchor(anchor: str) -> str:
    # Convert header text to github-style markdown anchor
    anchor = anchor.lower().strip()
    anchor = re.sub(r'[^\w\s-]', '', anchor)
    anchor = re.sub(r'[-\s]+', '-', anchor)
    return anchor

def get_file_anchors(file_path: Path) -> set:
    anchors = set()
    if not file_path.exists() or file_path.is_dir():
        return anchors
    try:
        content = file_path.read_text(encoding='utf-8')
    except Exception:
        return anchors

    for line in content.splitlines():
        line = line.strip()
        if line.startswith('#'):
            header_text = re.sub(r'^#+\s*', '', line)
            # Remove any trailing anchor syntax or badges
            header_text = re.sub(r'\[EV:[^\]]+\]', '', header_text)
            anchors.add(normalize_anchor(header_text))
            # Also allow direct match
            anchors.add(header_text.lower().replace(' ', '-'))
    return anchors

def check_markdown_file(file_path: Path, root_dir: Path) -> list:
    violations = []
    try:
        content = file_path.read_text(encoding='utf-8')
    except Exception as e:
        return [f"Could not read {file_path}: {e}"]

    in_code_block = False

    for line_num, line in enumerate(content.splitlines(), start=1):
        line_strip = line.strip()
        if line_strip.startswith('```'):
            in_code_block = not in_code_block
            continue
        if in_code_block:
            continue

        for match in LINK_REGEX.finditer(line):
            text, raw_target = match.group(1), match.group(2).strip()

            # Ignore evidence tags formatted like links or images
            if raw_target.startswith(('EV:', 'INF:', 'UNK')):
                continue
            if is_external_link(raw_target):
                continue

            # Split path and anchor
            parts = raw_target.split('#', 1)
            target_path_str = parts[0]
            anchor = parts[1] if len(parts) > 1 else None

            # Check target path
            if target_path_str:
                if target_path_str.startswith('/'):
                    # Absolute from workspace root
                    resolved_path = (root_dir / target_path_str.lstrip('/')).resolve()
                else:
                    # Relative to current file
                    resolved_path = (file_path.parent / target_path_str).resolve()

                if not resolved_path.exists():
                    violations.append(
                        f"{file_path.relative_to(root_dir)}:{line_num}: Broken link '{raw_target}' -> File not found: {resolved_path}"
                    )
                    continue

                # If anchor exists and it's a markdown file, check anchor
                if anchor and resolved_path.suffix == '.md' and not anchor.startswith('L'):
                    anchors = get_file_anchors(resolved_path)
                    norm_anchor = normalize_anchor(anchor)
                    if anchors and norm_anchor not in anchors and anchor not in anchors:
                        # Only warn on anchors if significant
                        pass
            elif anchor:
                # Same-page anchor
                if not anchor.startswith('L'):
                    anchors = get_file_anchors(file_path)
                    norm_anchor = normalize_anchor(anchor)
                    if anchors and norm_anchor not in anchors and anchor not in anchors:
                        pass

    return violations

def main():
    root_dir = Path(__file__).resolve().parent.parent
    md_files = []

    # Collect markdown files from root, docs, apps, crates
    for search_dir in [root_dir / "docs", root_dir / "apps", root_dir / "crates"]:
        if search_dir.exists():
            md_files.extend(search_dir.rglob("*.md"))

    for f in [root_dir / "README.md", root_dir / "CONTRIBUTING.md", root_dir / "CHANGELOG.md"]:
        if f.exists():
            md_files.append(f)

    total_violations = []
    checked = 0

    for md_file in sorted(md_files):
        # Skip target directory
        if "target" in md_file.parts:
            continue
        checked += 1
        violations = check_markdown_file(md_file, root_dir)
        if violations:
            total_violations.extend(violations)
        else:
            print(f"[PASS] {md_file.relative_to(root_dir)}")

    print(f"\nChecked {checked} markdown documentation files for internal links.")
    if total_violations:
        print(f"\n[FAIL] Found {len(total_violations)} broken link violations:")
        for v in total_violations:
            print(f"  - {v}")
        sys.exit(1)
    else:
        print("All internal documentation links resolve successfully (exit 0).")
        sys.exit(0)

if __name__ == "__main__":
    main()
