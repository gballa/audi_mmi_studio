//! Audi MMI Studio Desktop native entry point (§15, ADR-002).

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-V") {
        println!("Audi MMI Studio Desktop v0.1.0");
        return;
    }

    if args.iter().any(|a| a == "--compile-map") {
        let profile_idx = args.iter().position(|a| a == "--profile").map(|i| i + 1);
        let profile_code = profile_idx
            .and_then(|i| args.get(i))
            .cloned()
            .unwrap_or_else(|| "AL".to_string());
        let output_idx = args.iter().position(|a| a == "--output").map(|i| i + 1);
        let output_dir = output_idx
            .and_then(|i| args.get(i))
            .cloned()
            .unwrap_or_else(|| "build/desktop_sd".to_string());

        println!("Executing headless map compilation for profile: {}...", profile_code);
        let request = mmi_studio_desktop::MapCompileIpcRequest {
            profile_code,
            enable_gmp: true,
            output_dir,
        };
        match mmi_studio_desktop::handle_compile_map_pipeline(request) {
            Ok(res) => {
                for log in &res.logs {
                    println!("{}", log);
                }
                println!(
                    "Compilation successful! Total pages: {}, Total bytes: {}, Root SHA-1: {}",
                    res.total_pages, res.total_bytes, res.root_sha1
                );
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error during map compilation: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("Audi MMI Studio Desktop Workstation v0.1.0");
    println!("Architecture: Native Desktop Shell (Tauri v2 / React 19)");
    println!("IPC Boundary: Strongly-typed JSON schema with QNX & FLDB engines");
}
