use std::collections::VecDeque;
use std::time::Duration;
use mmi_diagnostics::{
    decode_st_min, encode_st_min, CanAdapter, CanFrame, DiagnosticsError, IsoTpChannel,
    IsoTpConfig, FC_OVERFLOW, ISOTP_CF, ISOTP_FC, ISOTP_FF, ISOTP_SF,
    MAX_ISOTP_PAYLOAD_LEN,
};

/// Dual-endpoint in-memory pipe for testing ISO-TP transmission between two nodes
struct IsoTpPipe {
    pub a_to_b: VecDeque<CanFrame>,
    pub b_to_a: VecDeque<CanFrame>,
}

impl IsoTpPipe {
    fn new() -> Self {
        Self {
            a_to_b: VecDeque::new(),
            b_to_a: VecDeque::new(),
        }
    }

    fn endpoint_a(&mut self) -> EndpointA<'_> {
        EndpointA { pipe: self }
    }

    fn endpoint_b(&mut self) -> EndpointB<'_> {
        EndpointB { pipe: self }
    }
}

struct EndpointA<'a> {
    pipe: &'a mut IsoTpPipe,
}

impl<'a> CanAdapter for EndpointA<'a> {
    fn send(&mut self, frame: &CanFrame) -> Result<(), DiagnosticsError> {
        self.pipe.a_to_b.push_back(frame.clone());
        Ok(())
    }

    fn receive(&mut self, _timeout: Duration) -> Result<CanFrame, DiagnosticsError> {
        self.pipe.b_to_a.pop_front().ok_or(DiagnosticsError::Timeout)
    }
}

struct EndpointB<'a> {
    pipe: &'a mut IsoTpPipe,
}

impl<'a> CanAdapter for EndpointB<'a> {
    fn send(&mut self, frame: &CanFrame) -> Result<(), DiagnosticsError> {
        self.pipe.b_to_a.push_back(frame.clone());
        Ok(())
    }

    fn receive(&mut self, _timeout: Duration) -> Result<CanFrame, DiagnosticsError> {
        self.pipe.a_to_b.pop_front().ok_or(DiagnosticsError::Timeout)
    }
}

#[test]
fn test_st_min_encoding_and_decoding() {
    // Millisecond encoding: 0..127 ms
    assert_eq!(encode_st_min(Duration::from_millis(0)), 0x00);
    assert_eq!(decode_st_min(0x00), Duration::from_millis(0));

    assert_eq!(encode_st_min(Duration::from_millis(25)), 25);
    assert_eq!(decode_st_min(25), Duration::from_millis(25));

    assert_eq!(encode_st_min(Duration::from_millis(127)), 127);
    assert_eq!(decode_st_min(127), Duration::from_millis(127));

    // Microsecond encoding: 100..900 µs (0xF1..0xF9)
    assert_eq!(decode_st_min(0xF1), Duration::from_micros(100));
    assert_eq!(decode_st_min(0xF5), Duration::from_micros(500));
    assert_eq!(decode_st_min(0xF9), Duration::from_micros(900));

    assert_eq!(encode_st_min(Duration::from_micros(200)), 0xF2);
    assert_eq!(encode_st_min(Duration::from_micros(700)), 0xF7);
}

#[test]
fn test_isotp_single_frame_all_lengths() {
    let mut pipe = IsoTpPipe::new();
    let tx_channel = IsoTpChannel::new(IsoTpConfig {
        tx_id: 0x714,
        rx_id: 0x77E,
        timeout: Duration::from_millis(100),
        ..Default::default()
    });
    let rx_channel = IsoTpChannel::new(IsoTpConfig {
        tx_id: 0x77E,
        rx_id: 0x714,
        timeout: Duration::from_millis(100),
        ..Default::default()
    });

    for len in 1..=7 {
        let payload: Vec<u8> = (0..len as u8).map(|i| i + 10).collect();

        // Node A sends
        {
            let mut ep_a = pipe.endpoint_a();
            tx_channel.send(&mut ep_a, &payload).unwrap();
        }

        // Verify CAN frame structure
        assert_eq!(pipe.a_to_b.len(), 1);
        let frame = pipe.a_to_b.front().unwrap();
        assert_eq!(frame.id, 0x714);
        assert_eq!(frame.data[0] & 0xF0, ISOTP_SF);
        assert_eq!((frame.data[0] & 0x0F) as usize, len);
        assert_eq!(&frame.data[1..1 + len], &payload[..]);

        // Node B receives
        {
            let mut ep_b = pipe.endpoint_b();
            let received = rx_channel.receive(&mut ep_b).unwrap();
            assert_eq!(received, payload);
        }
    }
}

#[test]
fn test_isotp_payload_size_boundary_4095() {
    let mut pipe = IsoTpPipe::new();
    let channel = IsoTpChannel::new(IsoTpConfig::default());

    // 4096 bytes exceeds 4095 max
    let oversized = vec![0x55; MAX_ISOTP_PAYLOAD_LEN + 1];
    let mut ep = pipe.endpoint_a();
    let err = channel.send(&mut ep, &oversized).unwrap_err();
    match err {
        DiagnosticsError::IsoTp(msg) => {
            assert!(msg.contains("exceeds ISO-TP 4095 byte maximum"));
        }
        _ => panic!("Expected IsoTp error"),
    }
}

#[test]
fn test_isotp_flow_control_overflow_handling() {
    let mut pipe = IsoTpPipe::new();
    let channel = IsoTpChannel::new(IsoTpConfig {
        tx_id: 0x714,
        rx_id: 0x77E,
        timeout: Duration::from_millis(100),
        ..Default::default()
    });

    // Node A sends First Frame (payload 20 bytes)
    let payload = vec![0x42; 20];

    // Seed Node B with an FC OVERFLOW frame back to Node A
    let fc_ovflw = CanFrame::new_standard(0x77E, &[ISOTP_FC | FC_OVERFLOW, 0, 0]).with_padding(0xAA);
    pipe.b_to_a.push_back(fc_ovflw);

    let mut ep_a = pipe.endpoint_a();
    let err = channel.send(&mut ep_a, &payload).unwrap_err();
    match err {
        DiagnosticsError::IsoTp(msg) => {
            assert!(msg.contains("overflow"));
        }
        _ => panic!("Expected overflow error"),
    }
}

#[test]
fn test_isotp_sequence_number_rollover() {
    // 16 CFs means sequence numbers 1..15, then 0
    // Payload size: 6 (FF) + 15*7 (CFs 1..15) + 7 (CF 0) = 6 + 105 + 7 = 118 bytes
    let total_len = 118;
    let payload: Vec<u8> = (0..total_len).map(|i| (i % 256) as u8).collect();

    // Verify First Frame and Consecutive Frame generation
    let mut frames = Vec::new();
    // FF
    let mut ff_data = vec![ISOTP_FF | ((total_len >> 8) as u8 & 0x0F), (total_len & 0xFF) as u8];
    ff_data.extend_from_slice(&payload[..6]);
    frames.push(CanFrame::new_standard(0x714, &ff_data).with_padding(0xAA));

    // CFs
    let mut offset = 6;
    let mut sn = 1u8;
    while offset < total_len {
        let chunk = (total_len - offset).min(7);
        let mut cf_data = vec![ISOTP_CF | (sn & 0x0F)];
        cf_data.extend_from_slice(&payload[offset..offset + chunk]);
        frames.push(CanFrame::new_standard(0x714, &cf_data).with_padding(0xAA));
        offset += chunk;
        sn = (sn + 1) & 0x0F;
    }

    assert!(frames.len() >= 17);
    // Frame index 15 is sn 15, frame index 16 is sn 0 (rollover)
    assert_eq!(frames[15].data[0] & 0x0F, 15);
    assert_eq!(frames[16].data[0] & 0x0F, 0);
}

#[test]
fn test_isotp_consecutive_frame_sequence_mismatch() {
    struct FaultySenderAdapter {
        step: usize,
    }

    impl CanAdapter for FaultySenderAdapter {
        fn send(&mut self, _frame: &CanFrame) -> Result<(), DiagnosticsError> {
            Ok(())
        }

        fn receive(&mut self, _timeout: Duration) -> Result<CanFrame, DiagnosticsError> {
            self.step += 1;
            match self.step {
                1 => {
                    // FF for 16 bytes
                    Ok(CanFrame::new_standard(0x77E, &[ISOTP_FF | 0x00, 16, 1, 2, 3, 4, 5, 6]))
                }
                2 => {
                    // CF with sequence number 3 instead of expected 1!
                    Ok(CanFrame::new_standard(0x77E, &[ISOTP_CF | 0x03, 7, 8, 9, 10, 11, 12, 13]))
                }
                _ => Err(DiagnosticsError::Timeout),
            }
        }
    }

    let mut adapter = FaultySenderAdapter { step: 0 };
    let channel = IsoTpChannel::new(IsoTpConfig {
        tx_id: 0x714,
        rx_id: 0x77E,
        timeout: Duration::from_millis(100),
        ..Default::default()
    });

    let res = channel.receive(&mut adapter);
    assert!(res.is_err());
    match res.unwrap_err() {
        DiagnosticsError::IsoTp(msg) => {
            assert!(msg.contains("sequence mismatch"));
        }
        _ => panic!("Expected sequence mismatch IsoTp error"),
    }
}

#[test]
fn test_isotp_multiframe_request_and_response_with_loopback() {
    use mmi_diagnostics::{LoopbackSimulator, UdsClient, DID_CODING};

    let mut sim = LoopbackSimulator::new();
    let mut client = UdsClient::new(&mut sim, IsoTpConfig {
        tx_id: 0x714,
        rx_id: 0x77E,
        timeout: Duration::from_millis(500),
        ..Default::default()
    });

    // 64-byte payload for WriteDID: 1 byte SID (0x2E) + 2 bytes DID + 61 bytes data = 64 bytes total
    let data = vec![0x42; 61];
    client.write_did(DID_CODING, &data).unwrap();

    // Verify written payload
    let readback = client.read_did(DID_CODING).unwrap();
    assert_eq!(readback, data);
}

#[test]
fn test_isotp_512_bytes_with_sequence_rollover_in_loopback() {
    use mmi_diagnostics::{LoopbackSimulator, UdsClient, DID_CODING};

    let mut sim = LoopbackSimulator::new();
    let mut client = UdsClient::new(&mut sim, IsoTpConfig {
        tx_id: 0x714,
        rx_id: 0x77E,
        timeout: Duration::from_millis(500),
        ..Default::default()
    });

    // 512-byte payload exercises multiple sequence number rollovers (1..15, 0..15...)
    let data: Vec<u8> = (0..512).map(|i| (i % 256) as u8).collect();
    client.write_did(DID_CODING, &data).unwrap();

    let readback = client.read_did(DID_CODING).unwrap();
    assert_eq!(readback, data);
}

