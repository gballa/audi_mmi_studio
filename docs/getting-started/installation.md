# Getting Started — Installation & Building

This guide walks through building the **Audi MMI Studio** toolchain from source using offline-first workflows.

---

## 1. Cloning the Repository

Clone the project repository to your workstation:

```bash
git clone <repository-url> audi_mmi_studio
cd audi_mmi_studio
```

Ensure the repository structure is intact:
```text
audi_mmi_studio/
├── apps/               # Headless CLI and Desktop GUI applications
├── crates/             # 12 modular Rust libraries
├── docs/               # Complete documentation system
├── recipes/            # Declarative theme recipes
├── scripts/            # Verification and utility scripts
├── Cargo.toml          # Workspace manifest
└── README.md           # Project entry point
```

---

## 2. Building the CLI (`mmi-studio-cli`)

The headless CLI tool is compiled using standard Cargo commands with `--offline`:

```bash
# Compile debug build (fast compilation)
cargo build -p mmi-studio-cli --offline

# Compile optimized release binary
cargo build --release -p mmi-studio-cli --offline
```

The compiled release binary is located at:
```text
target/release/mmi-studio-cli
```

Verify the binary execution:
```bash
./target/release/mmi-studio-cli --version
```

---

## 3. Building the Desktop GUI (`mmi-studio-desktop`)

The desktop application consists of a Tauri v2 native backend bridge and a React 18 / TypeScript frontend.

### Step 1: Install Frontend Dependencies
```bash
cd apps/mmi-studio-desktop
npm install
```

### Step 2: Build the TypeScript Frontend
```bash
npm run build
```

### Step 3: Run the Development Server (Browser Preview)
```bash
npm run dev
```
The browser preview runs at `http://localhost:5173` with local IPC mock adapters.

### Step 4: Package Native Desktop App (Optional)
If `tauri-cli` is installed:
```bash
cargo tauri build
```

---

## 4. Verifying the Entire Workspace

To verify compilation and run all unit and integration tests across the 14 workspace members:

```bash
# Run all tests strictly offline
cargo test --workspace --offline
```

All tests should pass with exit code 0.
