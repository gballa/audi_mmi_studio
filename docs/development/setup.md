# Development — Workspace Setup

This document guides developers through setting up the local engineering environment for contributing to **Audi MMI Studio**.

---

## 1. Prerequisites

Ensure your host machine has the following tools installed:
- **Rust Toolchain**: 1.80+ (`rustup toolchain install stable`).
- **Cargo Offline Mode**: Verified via `cargo build --offline`.
- **Node.js**: 18+ and `npm` (for the desktop UI frontend).
- **Python**: 3.9+ (for documentation and evidence tag verification scripts).

---

## 2. Setting Up the Offline Cache

To ensure builds can execute 100% offline without external network access:

```bash
# Verify cargo environment
cargo --version

# Ensure dependencies are available locally
cargo check --workspace --offline
```

If any dependency is reported missing, vendor dependencies or verify the cargo home cache (`$HOME/.cargo`).

---

## 3. Building the Entire Workspace

```bash
# Build all workspace crates and CLI in debug mode
cargo build --workspace --offline

# Build release CLI
cargo build --release -p mmi-studio-cli --offline
```

---

## 4. Setting Up the Desktop Frontend

```bash
cd apps/mmi-studio-desktop

# Install packages
npm install

# Test compilation
npm run build
```

---

## 5. Verification Pre-Flight

Run the all-in-one verification script to confirm your setup:

```bash
./scripts/verify-workstation.sh
```
If all 5 stages exit with `0`, your development environment is fully operational.
