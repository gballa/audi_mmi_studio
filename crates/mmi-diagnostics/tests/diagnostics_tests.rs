use mmi_diagnostics::{
    CanFrame, IsoTpChannel, IsoTpConfig, LoopbackSimulator, SvmSolver, UdsClient,
    DID_ADAPTATION_CHANNEL_15, DID_GREEN_MENU_ENABLE,
    SVM_CHANNEL_15_XOR_KEY,
};

#[test]
fn test_can_frame_creation_and_padding() {
    let frame = CanFrame::new_standard(0x714, &[0x02, 0x10, 0x03]).with_padding(0xAA);
    assert_eq!(frame.id, 0x714);
    assert_eq!(frame.data.len(), 8);
    assert_eq!(&frame.data[0..3], &[0x02, 0x10, 0x03]);
    assert_eq!(&frame.data[3..8], &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
}

#[test]
fn test_isotp_single_frame_exchange() {
    let mut loopback = LoopbackSimulator::new();
    let channel = IsoTpChannel::new(IsoTpConfig::default());

    // Send DiagnosticSessionControl(Extended)
    let req = [0x10, 0x03];
    channel.send(&mut loopback, &req).expect("ISO-TP send failed");
    let resp = channel.receive(&mut loopback).expect("ISO-TP receive failed");

    // Expected response: 0x50 0x03 ...
    assert!(!resp.is_empty());
    assert_eq!(resp[0], 0x50);
    assert_eq!(resp[1], 0x03);
}

#[test]
fn test_uds_client_read_and_write_did() {
    let mut loopback = LoopbackSimulator::new();
    let mut client = UdsClient::new(&mut loopback, IsoTpConfig::default());

    // Read default Channel 15 (24576 = 0x6000)
    let ch15 = client.read_did(DID_ADAPTATION_CHANNEL_15).expect("Read DID failed");
    assert_eq!(ch15, 24576u16.to_be_bytes());

    // Write new value
    let new_val: u16 = 43850;
    client.write_did(DID_ADAPTATION_CHANNEL_15, &new_val.to_be_bytes())
        .expect("Write DID failed");

    // Read back and verify
    let ch15_updated = client.read_did(DID_ADAPTATION_CHANNEL_15).expect("Read back failed");
    assert_eq!(ch15_updated, new_val.to_be_bytes());
}

#[test]
fn test_svm_fault_clearance_end_to_end() {
    let mut loopback = LoopbackSimulator::new();
    let initial_val: u16 = 24576;
    loopback.set_channel_15(initial_val);

    let res = SvmSolver::resolve_svm(&mut loopback, true).expect("SVM resolution failed");

    assert!(res.connected);
    assert_eq!(res.original_channel_15, initial_val);
    assert_eq!(res.updated_channel_15, initial_val ^ SVM_CHANNEL_15_XOR_KEY);
    assert!(res.dtc_03276_cleared);
    assert!(res.gem_unlocked);

    // Verify in loopback state
    assert_eq!(loopback.get_channel_15(), initial_val ^ SVM_CHANNEL_15_XOR_KEY);
    assert_eq!(loopback.dids.get(&DID_GREEN_MENU_ENABLE).unwrap(), &[0x01]);
    assert!(loopback.active_dtcs.is_empty());
}
