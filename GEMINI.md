# Antigravity Project Constitution

## 1. Operating Axioms
- Autonomous execution requires proactive verification. Never emit "done" without executing tests and proving correctness.
- Read only what is necessary, modify only what is specified, verify everything that matters.
- Never refactor surrounding architecture unless explicitly instructed by an Implementation Plan.

## 2. Environment & Validation Toolchain
- Runtime: Node.js (TypeScript) / Python (Poetry) / Rust (Cargo)
- Primary Validation Commands:
  - Compile: `cargo build --release -p mmi-studio-cli --offline` || `poetry run mypy src/` || `cargo check`
  - Lint: `pnpm run lint` || `poetry run ruff check .`
  - Test: `cargo test -p mmi-studio-cli --test e2e_map_pipeline` || `cargo test --test e2e_map_pipeline` || `cargo test`

## 3. Autonomous Modification Rules
- Bounded Edits: Use `replace_file_content` for surgical updates. Avoid full-file rewrites via `write_to_file`.
- Execution Sandbox: Run shell commands inside the sandbox. Unsandboxed executions require explicit user approval.
- Git Safety:
  - Work on isolated branches or Git worktrees (`agy` New Worktree Mode).
  - Do not alter package lockfiles without documented dependency intent.
  - Never run `git reset --hard` or `git checkout -- .` without confirmation.

## 4. Handling Ambiguity & Failures
- If specifications are ambiguous or requirements contradict existing code, halt and request clarification; do not invent APIs.
- When compilation or tests fail:
  1. Capture the exact terminal output and stack trace.
  2. Isolate the failure cause without altering unrelated source files.
  3. Formulate an atomic fix and re-run the specific test.
  4. Abort after 3 consecutive failed cycles and request human intervention.

## 5. Definition of Done (DoD)
All modifications must satisfy:
1. Clean compilation and zero static analysis warnings.
2. 100% pass rate on new and existing regression tests.
3. Git diff inspection confirming no unintended file changes or secrets.
4. An Implementation Walkthrough artifact summarizing changes and verification logs.
