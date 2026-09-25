use mmi_diagnostics::{
    CanAdapter, CanFrame, DiagnosticsError, LoopbackSimulator, SerialElmAdapter, SocketCanAdapter,
};

#[test]
fn test_loopback_simulator_initial_state() {
    let sim = LoopbackSimulator::new();
    assert_eq!(sim.active_session, 0x01);
    assert!(!sim.security_unlocked);
    assert_eq!(sim.get_channel_15(), 24576);
    assert_eq!(sim.get_car_menu_setting(), 5);
    assert!(!sim.is_gem_unlocked());
    assert!(sim.is_dtc_present(0x03276));
    assert!(sim.is_dtc_present(0x03175));
    assert_eq!(sim.name(), "LoopbackSimulator");
}

#[test]
fn test_serial_elm_adapter_format_tx() {
    let frame1 = CanFrame::new_standard(0x714, &[0x02, 0x10, 0x03]);
    let tx_str1 = SerialElmAdapter::format_frame_tx(&frame1);
    assert_eq!(tx_str1, "021003\r");

    let frame2 = CanFrame::new_standard(0x714, &[0x03, 0x22, 0x06, 0x15]);
    let tx_str2 = SerialElmAdapter::format_frame_tx(&frame2);
    assert_eq!(tx_str2, "03220615\r");
}

#[test]
fn test_serial_elm_adapter_parse_responses() {
    // 1. Standard raw response with spaces and prompt
    let raw1 = "06 50 03 00 32 01 F4\r\r>";
    let frames1 = SerialElmAdapter::parse_elm_response(raw1, 0x77E).unwrap();
    assert_eq!(frames1.len(), 1);
    assert_eq!(frames1[0].id, 0x77E);
    assert_eq!(frames1[0].data, vec![0x06, 0x50, 0x03, 0x00, 0x32, 0x01, 0xF4]);

    // 2. Response with CAN ID header (77E)
    let raw2 = "77E 03 7F 22 31\r\r>";
    let frames2 = SerialElmAdapter::parse_elm_response(raw2, 0x77E).unwrap();
    assert_eq!(frames2.len(), 1);
    assert_eq!(frames2[0].id, 0x77E);
    assert_eq!(frames2[0].data, vec![0x03, 0x7F, 0x22, 0x31]);

    // 3. Response with noise and status words
    let raw3 = "SEARCHING...\r\n02 50 01\r\n>";
    let frames3 = SerialElmAdapter::parse_elm_response(raw3, 0x77E).unwrap();
    assert_eq!(frames3.len(), 1);
    assert_eq!(frames3[0].data, vec![0x02, 0x50, 0x01]);

    // 4. CAN error detection
    let raw_err = "CAN ERROR\r\n>";
    let err = SerialElmAdapter::parse_elm_response(raw_err, 0x77E).unwrap_err();
    match err {
        DiagnosticsError::Can(msg) => assert!(msg.contains("hardware error")),
        _ => panic!("Expected Can error"),
    }

    // 5. Buffer full detection
    let raw_buf = "BUFFER FULL\r\n>";
    let err_buf = SerialElmAdapter::parse_elm_response(raw_buf, 0x77E).unwrap_err();
    match err_buf {
        DiagnosticsError::IsoTp(msg) => assert!(msg.contains("overflow")),
        _ => panic!("Expected IsoTp buffer overflow error"),
    }
}

#[test]
fn test_socketcan_adapter_instantiation_and_platform_guard() {
    let mut adapter = SocketCanAdapter::new("vcan0");
    assert_eq!(adapter.interface_name, "vcan0");
    assert_eq!(adapter.name(), "SocketCanAdapter");

    #[cfg(not(target_os = "linux"))]
    {
        // On macOS, open should return PlatformNotSupported cleanly
        let res = adapter.open();
        assert!(res.is_err());
        match res.unwrap_err() {
            DiagnosticsError::PlatformNotSupported(msg) => {
                assert!(msg.contains("Linux"));
            }
            _ => panic!("Expected PlatformNotSupported"),
        }
    }
}
