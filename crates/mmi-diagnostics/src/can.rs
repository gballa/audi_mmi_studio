use std::time::Duration;
use crate::error::DiagnosticsError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanFrame {
    pub id: u32,
    pub extended: bool,
    pub data: Vec<u8>,
    pub rtr: bool,
}

impl CanFrame {
    pub fn new_standard(id: u32, data: &[u8]) -> Self {
        assert!(id <= 0x7FF, "Standard CAN ID must be <= 0x7FF");
        assert!(data.len() <= 8, "CAN 2.0 payload must be <= 8 bytes");
        Self {
            id,
            extended: false,
            data: data.to_vec(),
            rtr: false,
        }
    }

    pub fn new_extended(id: u32, data: &[u8]) -> Self {
        assert!(id <= 0x1FFFFFFF, "Extended CAN ID must be <= 0x1FFFFFFF");
        assert!(data.len() <= 8, "CAN 2.0 payload must be <= 8 bytes");
        Self {
            id,
            extended: true,
            data: data.to_vec(),
            rtr: false,
        }
    }

    pub fn dlc(&self) -> usize {
        self.data.len()
    }

    /// Pads the frame payload with a filler byte (typically 0xAA or 0x00) up to 8 bytes.
    pub fn with_padding(mut self, pad_byte: u8) -> Self {
        while self.data.len() < 8 {
            self.data.push(pad_byte);
        }
        self
    }
}

pub trait CanAdapter: Send + Sync {
    fn send(&mut self, frame: &CanFrame) -> Result<(), DiagnosticsError>;
    fn receive(&mut self, timeout: Duration) -> Result<CanFrame, DiagnosticsError>;
    fn set_filter(&mut self, rx_id: u32, mask: u32) -> Result<(), DiagnosticsError> {
        let _ = (rx_id, mask);
        Ok(())
    }
    fn flush(&mut self) -> Result<(), DiagnosticsError> {
        Ok(())
    }
    fn name(&self) -> &str {
        "CanAdapter"
    }
}
