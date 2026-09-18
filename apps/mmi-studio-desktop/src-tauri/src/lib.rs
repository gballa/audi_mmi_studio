//! Audi MMI Studio Desktop Application core and IPC bridge (§15, ADR-002).

pub mod ipc;

pub use ipc::{
    handle_entropy, handle_hexdump, handle_inspect_file, handle_render_screen, EntropyResult,
    HexDumpResult, InspectResult, ScreenRenderResult,
};
