//! Audi MMI Studio Desktop native entry point (§15, ADR-002).

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-V") {
        println!("Audi MMI Studio Desktop v0.1.0");
        return;
    }

    println!("Audi MMI Studio Desktop Workstation v0.1.0");
    println!("Architecture: Native Desktop Shell (Tauri v2 / React 19)");
    println!("IPC Boundary: Strongly-typed JSON schema with QNX & FLDB engines");
}
