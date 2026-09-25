use mmi_diagnostics::{
    nrc_description, DiagnosticsError, LoopbackSimulator, UdsClient,
    DID_ADAPTATION_CHANNEL_15, DID_CAR_MENU_CONFIG, DID_CODING, DID_ECU_PART_NUMBER,
    DID_GREEN_MENU_ENABLE, DID_SOFTWARE_VERSION, DID_VIN, SESSION_DEFAULT, SESSION_EXTENDED,
    SESSION_PROGRAMMING,
};

#[test]
fn test_uds_session_control() {
    let mut sim = LoopbackSimulator::new();

    {
        let mut client = UdsClient::new(&mut sim, Default::default());

        // Enter Extended Session
        let resp = client.session_control(SESSION_EXTENDED).unwrap();
        assert_eq!(resp.len(), 5);
        assert_eq!(resp[0], SESSION_EXTENDED);
    }
    assert_eq!(sim.active_session, SESSION_EXTENDED);

    {
        let mut client = UdsClient::new(&mut sim, Default::default());

        // Switch to Programming Session
        let resp = client.session_control(SESSION_PROGRAMMING).unwrap();
        assert_eq!(resp[0], SESSION_PROGRAMMING);
    }
    assert_eq!(sim.active_session, SESSION_PROGRAMMING);

    {
        let mut client = UdsClient::new(&mut sim, Default::default());

        // Switch back to Default Session
        let resp = client.session_control(SESSION_DEFAULT).unwrap();
        assert_eq!(resp[0], SESSION_DEFAULT);
    }
    assert_eq!(sim.active_session, SESSION_DEFAULT);
}

#[test]
fn test_uds_read_standard_identification_dids() {
    let mut sim = LoopbackSimulator::new();
    let mut client = UdsClient::new(&mut sim, Default::default());

    // Part Number DID 0xF187
    let part_no = client.read_did(DID_ECU_PART_NUMBER).unwrap();
    assert_eq!(String::from_utf8_lossy(&part_no), "8R0035670");

    // Software Version DID 0xF189
    let sw_ver = client.read_did(DID_SOFTWARE_VERSION).unwrap();
    assert_eq!(String::from_utf8_lossy(&sw_ver), "HN+R_EU_AU_K0942_4");

    // VIN DID 0xF190
    let vin = client.read_did(DID_VIN).unwrap();
    assert_eq!(String::from_utf8_lossy(&vin), "WAUZZZ8K0DA123456");

    // Unknown DID returns NRC 0x31 (RequestOutOfRange)
    let err = client.read_did(0x9999).unwrap_err();
    match err {
        DiagnosticsError::NegativeResponse { service_id, nrc, description } => {
            assert_eq!(service_id, 0x22);
            assert_eq!(nrc, 0x31);
            assert_eq!(description, "RequestOutOfRange");
        }
        _ => panic!("Expected NegativeResponse with NRC 0x31"),
    }
}

#[test]
fn test_uds_write_did_and_readback() {
    let mut sim = LoopbackSimulator::new();
    let test_val: u16 = 51620;
    let new_car_menu: u16 = 6;

    {
        let mut client = UdsClient::new(&mut sim, Default::default());

        // Enter Extended Session
        client.session_control(SESSION_EXTENDED).unwrap();

        // Write Channel 15
        client.write_did(DID_ADAPTATION_CHANNEL_15, &test_val.to_be_bytes()).unwrap();

        // Verify Read
        let readback = client.read_did(DID_ADAPTATION_CHANNEL_15).unwrap();
        assert_eq!(readback, test_val.to_be_bytes());

        // Write Green Engineering Menu unlock (DID 0x0606 & 0x0611)
        client.write_did(DID_CODING, &[0x01]).unwrap();
        client.write_did(DID_GREEN_MENU_ENABLE, &[0x01]).unwrap();

        // Write Car Menu configuration (DID 0x0620)
        client.write_did(DID_CAR_MENU_CONFIG, &new_car_menu.to_be_bytes()).unwrap();
    }

    assert_eq!(sim.get_channel_15(), test_val);
    assert!(sim.is_gem_unlocked());
    assert_eq!(sim.get_car_menu_setting(), 6);
}

#[test]
fn test_uds_security_access_seed_key_flow() {
    let mut sim = LoopbackSimulator::new();
    assert!(!sim.security_unlocked);

    {
        let mut client = UdsClient::new(&mut sim, Default::default());

        // Successful security access with key = seed ^ 0xAA
        client.security_access(0x01, |seed| {
            seed.iter().map(|b| b ^ 0xAA).collect()
        }).unwrap();
    }

    assert!(sim.security_unlocked);
}

#[test]
fn test_uds_security_access_invalid_key_rejection() {
    let mut sim = LoopbackSimulator::new();

    {
        let mut client = UdsClient::new(&mut sim, Default::default());

        // Wrong key returns NegativeResponse (NRC 0x35 InvalidKey)
        let err = client.security_access(0x01, |_seed| {
            vec![0xFF, 0xFF, 0xFF, 0xFF]
        }).unwrap_err();

        match err {
            DiagnosticsError::NegativeResponse { service_id, nrc, description } => {
                assert_eq!(service_id, 0x27);
                assert_eq!(nrc, 0x35);
                assert_eq!(description, "InvalidKey");
            }
            _ => panic!("Expected NegativeResponse InvalidKey, got {:?}", err),
        }
    }

    assert!(!sim.security_unlocked);
}

#[test]
fn test_uds_security_access_vag_login() {
    let mut sim = LoopbackSimulator::new();

    {
        let mut client = UdsClient::new(&mut sim, Default::default());

        // Standard VAG Module 5F login code: 20103
        client.security_access_login(0x01, 20103).unwrap();
    }

    assert!(sim.security_unlocked);
}

#[test]
fn test_uds_clear_dtcs() {
    let mut sim = LoopbackSimulator::new();

    assert!(sim.is_dtc_present(0x03276));
    assert!(sim.is_dtc_present(0x03175));

    {
        let mut client = UdsClient::new(&mut sim, Default::default());
        client.clear_dtcs(0xFFFFFF).unwrap();
    }

    assert!(!sim.is_dtc_present(0x03276));
    assert!(!sim.is_dtc_present(0x03175));
    assert!(sim.active_dtcs.is_empty());
}

#[test]
fn test_uds_tester_present() {
    let mut sim = LoopbackSimulator::new();
    let mut client = UdsClient::new(&mut sim, Default::default());

    // With response
    client.tester_present(false).unwrap();

    // Suppress positive response
    client.tester_present(true).unwrap();
}

#[test]
fn test_nrc_descriptions() {
    assert_eq!(nrc_description(0x10), "GeneralReject");
    assert_eq!(nrc_description(0x11), "ServiceNotSupported");
    assert_eq!(nrc_description(0x12), "SubFunctionNotSupported");
    assert_eq!(nrc_description(0x13), "IncorrectMessageLengthOrInvalidFormat");
    assert_eq!(nrc_description(0x22), "ConditionsNotCorrect");
    assert_eq!(nrc_description(0x31), "RequestOutOfRange");
    assert_eq!(nrc_description(0x33), "SecurityAccessDenied");
    assert_eq!(nrc_description(0x35), "InvalidKey");
    assert_eq!(nrc_description(0x78), "RequestCorrectlyReceived-ResponsePending");
    assert_eq!(nrc_description(0xEE), "UnknownNRC");
}
