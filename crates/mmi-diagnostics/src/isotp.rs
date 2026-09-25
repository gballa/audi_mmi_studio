use std::time::{Duration, Instant};
use crate::can::{CanAdapter, CanFrame};
use crate::error::DiagnosticsError;

pub const ISOTP_SF: u8 = 0x00;
pub const ISOTP_FF: u8 = 0x10;
pub const ISOTP_CF: u8 = 0x20;
pub const ISOTP_FC: u8 = 0x30;

pub const FC_CTS: u8 = 0x00;
pub const FC_WAIT: u8 = 0x01;
pub const FC_OVERFLOW: u8 = 0x02;

pub const MAX_ISOTP_PAYLOAD_LEN: usize = 4095;

/// Encodes a Duration separation time into an ISO-TP STmin byte
pub fn encode_st_min(duration: Duration) -> u8 {
    let micros = duration.as_micros();
    if (100..=900).contains(&micros) {
        0xF0 + ((micros / 100) as u8)
    } else {
        let millis = duration.as_millis();
        if millis <= 127 {
            millis as u8
        } else {
            127 // cap at 127 ms
        }
    }
}

/// Decodes an ISO-TP STmin byte into a Duration
pub fn decode_st_min(byte: u8) -> Duration {
    if byte <= 0x7F {
        Duration::from_millis(byte as u64)
    } else if (0xF1..=0xF9).contains(&byte) {
        Duration::from_micros(((byte - 0xF0) as u64) * 100)
    } else {
        Duration::from_millis(0)
    }
}

#[derive(Debug, Clone)]
pub struct IsoTpConfig {
    pub tx_id: u32,
    pub rx_id: u32,
    pub timeout: Duration,
    pub block_size: u8,
    pub st_min: Duration,
    pub padding_byte: Option<u8>,
    pub max_wft: u32,
}

impl Default for IsoTpConfig {
    fn default() -> Self {
        Self {
            tx_id: 0x714, // MMI Module 5F Request ID
            rx_id: 0x77E, // MMI Module 5F Response ID
            timeout: Duration::from_millis(1000),
            block_size: 0, // 0 = unlimited block size
            st_min: Duration::from_millis(0),
            padding_byte: Some(0xAA),
            max_wft: 16,
        }
    }
}

pub struct IsoTpChannel {
    pub config: IsoTpConfig,
}

impl IsoTpChannel {
    pub fn new(config: IsoTpConfig) -> Self {
        Self { config }
    }

    /// Transmit a multi-byte UDS/ISO-TP payload over CAN
    pub fn send(&self, adapter: &mut dyn CanAdapter, payload: &[u8]) -> Result<(), DiagnosticsError> {
        let len = payload.len();
        if len == 0 {
            return Err(DiagnosticsError::IsoTp("Empty payload".into()));
        }

        if len <= 7 {
            // Single Frame (SF)
            let mut data = Vec::with_capacity(8);
            data.push(ISOTP_SF | (len as u8));
            data.extend_from_slice(payload);
            let mut frame = CanFrame::new_standard(self.config.tx_id, &data);
            if let Some(pad) = self.config.padding_byte {
                frame = frame.with_padding(pad);
            }
            adapter.send(&frame)?;
            return Ok(());
        }

        if len > MAX_ISOTP_PAYLOAD_LEN {
            return Err(DiagnosticsError::IsoTp(format!(
                "Payload exceeds ISO-TP 4095 byte maximum (length: {})",
                len
            )));
        }

        // Multi-frame: First Frame (FF)
        let mut ff_data = Vec::with_capacity(8);
        ff_data.push(ISOTP_FF | ((len >> 8) as u8 & 0x0F));
        ff_data.push((len & 0xFF) as u8);
        ff_data.extend_from_slice(&payload[..6]);

        let mut ff_frame = CanFrame::new_standard(self.config.tx_id, &ff_data);
        if let Some(pad) = self.config.padding_byte {
            ff_frame = ff_frame.with_padding(pad);
        }
        adapter.send(&ff_frame)?;

        // Wait for Flow Control (FC) frame from receiver
        let (mut block_size, mut st_min) = self.wait_for_flow_control(adapter)?;

        // Send Consecutive Frames (CF)
        let mut offset = 6;
        let mut sn: u8 = 1;
        let mut frames_in_block = 0u8;

        while offset < len {
            let chunk_len = (len - offset).min(7);
            let mut cf_data = Vec::with_capacity(8);
            cf_data.push(ISOTP_CF | (sn & 0x0F));
            cf_data.extend_from_slice(&payload[offset..offset + chunk_len]);

            let mut cf_frame = CanFrame::new_standard(self.config.tx_id, &cf_data);
            if let Some(pad) = self.config.padding_byte {
                cf_frame = cf_frame.with_padding(pad);
            }
            adapter.send(&cf_frame)?;

            offset += chunk_len;
            sn = (sn + 1) & 0x0F;
            frames_in_block += 1;

            if block_size > 0 && frames_in_block >= block_size && offset < len {
                // Block boundary reached: wait for next Flow Control frame
                let (new_bs, new_st) = self.wait_for_flow_control(adapter)?;
                block_size = new_bs;
                st_min = new_st;
                frames_in_block = 0;
            }

            if !st_min.is_zero() {
                std::thread::sleep(st_min);
            }
        }

        Ok(())
    }

    fn wait_for_flow_control(&self, adapter: &mut dyn CanAdapter) -> Result<(u8, Duration), DiagnosticsError> {
        let start = Instant::now();
        let mut wft_count = 0;

        loop {
            if start.elapsed() > self.config.timeout {
                return Err(DiagnosticsError::Timeout);
            }

            let resp = adapter.receive(self.config.timeout)?;
            if resp.id != self.config.rx_id || resp.data.is_empty() {
                continue;
            }

            let pci = resp.data[0] & 0xF0;
            if pci == ISOTP_FC {
                let flow_status = resp.data[0] & 0x0F;
                match flow_status {
                    FC_CTS => {
                        let bs = if resp.data.len() > 1 { resp.data[1] } else { 0 };
                        let st_val = if resp.data.len() > 2 { resp.data[2] } else { 0 };
                        let st = decode_st_min(st_val);
                        return Ok((bs, st));
                    }
                    FC_WAIT => {
                        wft_count += 1;
                        if wft_count > self.config.max_wft {
                            return Err(DiagnosticsError::IsoTp("Exceeded maximum Flow Control WAIT frames".into()));
                        }
                        continue;
                    }
                    FC_OVERFLOW => {
                        return Err(DiagnosticsError::IsoTp("Receiver buffer overflow reported via FC".into()));
                    }
                    _ => {
                        return Err(DiagnosticsError::IsoTp(format!(
                            "Invalid Flow Control status: 0x{:02X}",
                            flow_status
                        )));
                    }
                }
            }
        }
    }

    /// Receive and reassemble a multi-byte UDS/ISO-TP payload from CAN
    pub fn receive(&self, adapter: &mut dyn CanAdapter) -> Result<Vec<u8>, DiagnosticsError> {
        let start = Instant::now();

        loop {
            if start.elapsed() > self.config.timeout {
                return Err(DiagnosticsError::Timeout);
            }

            let frame = adapter.receive(self.config.timeout)?;
            if frame.id != self.config.rx_id || frame.data.is_empty() {
                continue;
            }

            let pci_type = frame.data[0] & 0xF0;

            match pci_type {
                ISOTP_SF => {
                    let len = (frame.data[0] & 0x0F) as usize;
                    if len == 0 || len > 7 || len >= frame.data.len() {
                        return Err(DiagnosticsError::IsoTp(format!("Invalid SF length: {}", len)));
                    }
                    return Ok(frame.data[1..1 + len].to_vec());
                }
                ISOTP_FF => {
                    if frame.data.len() < 8 {
                        return Err(DiagnosticsError::IsoTp("FF frame payload too short (need 8 bytes)".into()));
                    }
                    let total_len = (((frame.data[0] & 0x0F) as usize) << 8) | (frame.data[1] as usize);
                    if total_len <= 7 {
                        return Err(DiagnosticsError::IsoTp(format!(
                            "Invalid FF length (must be > 7): {}",
                            total_len
                        )));
                    }

                    if total_len > MAX_ISOTP_PAYLOAD_LEN {
                        // Send FC Overflow
                        let fc_ovflw = vec![ISOTP_FC | FC_OVERFLOW, 0, 0];
                        let mut fc_frame = CanFrame::new_standard(self.config.tx_id, &fc_ovflw);
                        if let Some(pad) = self.config.padding_byte {
                            fc_frame = fc_frame.with_padding(pad);
                        }
                        let _ = adapter.send(&fc_frame);
                        return Err(DiagnosticsError::IsoTp(format!(
                            "FF length exceeds maximum 4095 bytes: {}",
                            total_len
                        )));
                    }

                    let mut reassembled = Vec::with_capacity(total_len);
                    reassembled.extend_from_slice(&frame.data[2..8]);

                    // Send initial Flow Control (CTS, block_size, st_min)
                    let st_byte = encode_st_min(self.config.st_min);
                    let fc_data = vec![ISOTP_FC | FC_CTS, self.config.block_size, st_byte];
                    let mut fc_frame = CanFrame::new_standard(self.config.tx_id, &fc_data);
                    if let Some(pad) = self.config.padding_byte {
                        fc_frame = fc_frame.with_padding(pad);
                    }
                    adapter.send(&fc_frame)?;

                    let mut expected_sn: u8 = 1;
                    let mut frames_in_block = 0u8;
                    let cf_start = Instant::now();

                    while reassembled.len() < total_len {
                        if cf_start.elapsed() > self.config.timeout {
                            return Err(DiagnosticsError::Timeout);
                        }

                        let cf = adapter.receive(self.config.timeout)?;
                        if cf.id != self.config.rx_id || cf.data.is_empty() {
                            continue;
                        }

                        if (cf.data[0] & 0xF0) != ISOTP_CF {
                            continue;
                        }

                        let sn = cf.data[0] & 0x0F;
                        if sn != expected_sn {
                            return Err(DiagnosticsError::IsoTp(format!(
                                "CF sequence mismatch: expected {}, got {}",
                                expected_sn, sn
                            )));
                        }

                        expected_sn = (expected_sn + 1) & 0x0F;
                        let needed = total_len - reassembled.len();
                        let available = cf.data.len() - 1;
                        let take = needed.min(available);
                        reassembled.extend_from_slice(&cf.data[1..1 + take]);
                        frames_in_block += 1;

                        if self.config.block_size > 0
                            && frames_in_block >= self.config.block_size
                            && reassembled.len() < total_len
                        {
                            // Send subsequent Flow Control CTS
                            let fc_data = vec![ISOTP_FC | FC_CTS, self.config.block_size, st_byte];
                            let mut fc_frame = CanFrame::new_standard(self.config.tx_id, &fc_data);
                            if let Some(pad) = self.config.padding_byte {
                                fc_frame = fc_frame.with_padding(pad);
                            }
                            adapter.send(&fc_frame)?;
                            frames_in_block = 0;
                        }
                    }

                    return Ok(reassembled);
                }
                _ => {
                    // Ignore unexpected frames
                    continue;
                }
            }
        }
    }
}
