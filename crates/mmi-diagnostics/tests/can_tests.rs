use mmi_diagnostics::{CanAdapter, CanFrame, DiagnosticsError};
use std::time::Duration;

struct MockAdapter {
    sent: Vec<CanFrame>,
}

impl MockAdapter {
    fn new() -> Self {
        Self { sent: Vec::new() }
    }
}

impl CanAdapter for MockAdapter {
    fn send(&mut self, frame: &CanFrame) -> Result<(), DiagnosticsError> {
        self.sent.push(frame.clone());
        Ok(())
    }

    fn receive(&mut self, _timeout: Duration) -> Result<CanFrame, DiagnosticsError> {
        Err(DiagnosticsError::Timeout)
    }

    fn name(&self) -> &str {
        "MockAdapter"
    }
}

#[test]
fn test_standard_can_frame_creation() {
    let frame = CanFrame::new_standard(0x714, &[0x02, 0x10, 0x03]);
    assert_eq!(frame.id, 0x714);
    assert!(!frame.extended);
    assert_eq!(frame.dlc(), 3);
    assert_eq!(frame.data, vec![0x02, 0x10, 0x03]);
}

#[test]
fn test_extended_can_frame_creation() {
    let frame = CanFrame::new_extended(0x18DAF110, &[0x03, 0x22, 0xF1, 0x90]);
    assert_eq!(frame.id, 0x18DAF110);
    assert!(frame.extended);
    assert_eq!(frame.dlc(), 4);
    assert_eq!(frame.data, vec![0x03, 0x22, 0xF1, 0x90]);
}

#[test]
fn test_can_frame_padding() {
    let frame = CanFrame::new_standard(0x714, &[0x01, 0x3E]).with_padding(0xAA);
    assert_eq!(frame.dlc(), 8);
    assert_eq!(frame.data, vec![0x01, 0x3E, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);

    let zero_padded = CanFrame::new_standard(0x714, &[0x02, 0x10, 0x01]).with_padding(0x00);
    assert_eq!(zero_padded.dlc(), 8);
    assert_eq!(zero_padded.data, vec![0x02, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_mock_adapter_send_and_name() {
    let mut adapter = MockAdapter::new();
    assert_eq!(adapter.name(), "MockAdapter");

    let frame = CanFrame::new_standard(0x714, &[0x02, 0x10, 0x03]);
    adapter.send(&frame).unwrap();
    assert_eq!(adapter.sent.len(), 1);
    assert_eq!(adapter.sent[0], frame);
}
