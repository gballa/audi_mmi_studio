use mmi_formats::{GateVerdict, IdentityRebuildGate};
use std::path::Path;

#[test]
fn test_identity_rebuild_gate_on_real_precomp() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let precomp_path = workspace_dir
        .join("originals")
        .join("HN+R_EU_AU_K0942_4_[8R0906961FB]")
        .join("CombiStyles")
        .join("IND")
        .join("0")
        .join("default")
        .join("arr_e.precomp");

    if !precomp_path.exists() {
        return;
    }

    let data = std::fs::read(&precomp_path).unwrap();
    let report = IdentityRebuildGate::evaluate(
        "HN+R_EU_AU_K0942_4_[8R0906961FB]/CombiStyles/IND/0/default/arr_e.precomp",
        &data,
    )
    .expect("Gate evaluation failed");

    println!("Report: {:#?}", report);
    assert!(report.can_rebuild);
    assert!(matches!(
        report.verdict,
        GateVerdict::PassedBitForBit | GateVerdict::PassedCanonical { .. }
    ));
    assert_eq!(report.format_name, "precomp");
}

#[test]
fn test_identity_rebuild_gate_locks_signed_payload() {
    let dummy_data = b"SIGNED_PAYLOAD_CONTENT";
    let report = IdentityRebuildGate::evaluate("GEMMI/nav/models/MMI3GP_ECE_Hi_R_6_36_0.pkg", dummy_data)
        .expect("Gate evaluation failed");

    assert!(!report.can_rebuild);
    assert!(matches!(report.verdict, GateVerdict::LockedSignedArtefact { .. }));
}
