use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use crate::can::{CanAdapter, CanFrame};
use crate::error::DiagnosticsError;
use crate::isotp::{FC_CTS, ISOTP_CF, ISOTP_FC, ISOTP_FF, ISOTP_SF};
use crate::uds::*;

pub const MMI_REQ_ID: u32 = 0x714;
pub const MMI_RESP_ID: u32 = 0x77E;

/// High-fidelity in-memory simulator of Audi MMI Module 5F (Head Unit)
pub struct LoopbackSimulator {
    pub tx_queue: VecDeque<CanFrame>,
    pub rx_queue: VecDeque<CanFrame>,
    pub active_session: u8,
    pub security_unlocked: bool,
    pub dids: HashMap<u16, Vec<u8>>,
    pub active_dtcs: Vec<u32>,
    pub seed: [u8; 4],
    pub svm_03276_resolved: bool,
    pub svm_03175_resolved: bool,
    pub initial_car_menu_setting: u16,

    // ISO-TP reception state for incoming multi-frame requests
    rx_in_progress: bool,
    rx_expected_len: usize,
    rx_buffer: Vec<u8>,
    rx_expected_sn: u8,

    // Pending multi-frame response Consecutive Frames waiting for client Flow Control
    pending_cf_frames: VecDeque<CanFrame>,
}

impl Default for LoopbackSimulator {
    fn default() -> Self {
        let mut dids = HashMap::new();
        // Channel 15 default challenge value: 24576 (0x6000)
        dids.insert(DID_ADAPTATION_CHANNEL_15, 24576u16.to_be_bytes().to_vec());
        // Green Engineering Menu: 0 (disabled)
        dids.insert(DID_CODING, vec![0x00]);
        dids.insert(DID_GREEN_MENU_ENABLE, vec![0x00]);
        // Car Menu Configuration (Channel 32): default setting 5
        let initial_car_menu: u16 = 5;
        dids.insert(DID_CAR_MENU_CONFIG, initial_car_menu.to_be_bytes().to_vec());
        // Identification DIDs
        dids.insert(DID_ECU_PART_NUMBER, b"8R0035670".to_vec());
        dids.insert(DID_SOFTWARE_VERSION, b"HN+R_EU_AU_K0942_4".to_vec());
        dids.insert(DID_VIN, b"WAUZZZ8K0DA123456".to_vec());
        dids.insert(DID_SYSTEM_NAME, b"MMI3G+ High".to_vec());
        // Supply voltage: 13.92 V (13920 mV)
        dids.insert(DID_SUPPLY_VOLTAGE, 13920u16.to_be_bytes().to_vec());

        Self {
            tx_queue: VecDeque::new(),
            rx_queue: VecDeque::new(),
            active_session: SESSION_DEFAULT,
            security_unlocked: false,
            dids,
            active_dtcs: vec![0x03276, 0x03175],
            seed: [0x12, 0x34, 0x56, 0x78],
            svm_03276_resolved: false,
            svm_03175_resolved: false,
            initial_car_menu_setting: initial_car_menu,
            rx_in_progress: false,
            rx_expected_len: 0,
            rx_buffer: Vec::new(),
            rx_expected_sn: 1,
            pending_cf_frames: VecDeque::new(),
        }
    }
}

impl LoopbackSimulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a specific adaptation channel 15 challenge value
    pub fn set_channel_15(&mut self, val: u16) {
        self.dids.insert(DID_ADAPTATION_CHANNEL_15, val.to_be_bytes().to_vec());
    }

    /// Reads the current channel 15 value
    pub fn get_channel_15(&self) -> u16 {
        if let Some(bytes) = self.dids.get(&DID_ADAPTATION_CHANNEL_15) {
            if bytes.len() >= 2 {
                return u16::from_be_bytes([bytes[0], bytes[1]]);
            }
        }
        0
    }

    /// Reads the current car menu configuration setting
    pub fn get_car_menu_setting(&self) -> u16 {
        if let Some(bytes) = self.dids.get(&DID_CAR_MENU_CONFIG) {
            if bytes.len() >= 2 {
                return u16::from_be_bytes([bytes[0], bytes[1]]);
            }
        }
        0
    }

    /// Checks if Green Engineering Menu is unlocked
    pub fn is_gem_unlocked(&self) -> bool {
        self.dids
            .get(&DID_GREEN_MENU_ENABLE)
            .map(|v| v == &[0x01])
            .unwrap_or(false)
            || self
                .dids
                .get(&DID_CODING)
                .map(|v| v == &[0x01])
                .unwrap_or(false)
    }

    pub fn is_dtc_present(&self, dtc: u32) -> bool {
        self.active_dtcs.contains(&dtc)
    }

    fn queue_uds_response(&mut self, resp_payload: &[u8]) {
        if resp_payload.len() <= 7 {
            let mut resp_data = Vec::with_capacity(8);
            resp_data.push(ISOTP_SF | (resp_payload.len() as u8));
            resp_data.extend_from_slice(resp_payload);
            while resp_data.len() < 8 {
                resp_data.push(0xAA);
            }
            self.rx_queue.push_back(CanFrame::new_standard(MMI_RESP_ID, &resp_data));
        } else {
            // Multi-frame First Frame (FF)
            let total_len = resp_payload.len();
            let mut ff_data = Vec::with_capacity(8);
            ff_data.push(ISOTP_FF | ((total_len >> 8) as u8 & 0x0F));
            ff_data.push((total_len & 0xFF) as u8);
            ff_data.extend_from_slice(&resp_payload[..6]);
            self.rx_queue.push_back(CanFrame::new_standard(MMI_RESP_ID, &ff_data));

            // Prepare subsequent Consecutive Frames
            self.pending_cf_frames.clear();
            let mut offset = 6;
            let mut sn = 1u8;
            while offset < total_len {
                let chunk_len = (total_len - offset).min(7);
                let mut cf_data = Vec::with_capacity(8);
                cf_data.push(ISOTP_CF | (sn & 0x0F));
                cf_data.extend_from_slice(&resp_payload[offset..offset + chunk_len]);
                while cf_data.len() < 8 {
                    cf_data.push(0xAA);
                }
                self.pending_cf_frames.push_back(CanFrame::new_standard(MMI_RESP_ID, &cf_data));
                offset += chunk_len;
                sn = (sn + 1) & 0x0F;
            }
        }
    }

    fn process_uds_request(&mut self, request: &[u8]) -> Vec<u8> {
        if request.is_empty() {
            return vec![SID_NEGATIVE_RESPONSE, 0x00, 0x13];
        }

        let sid = request[0];
        match sid {
            SID_DIAGNOSTIC_SESSION_CONTROL => {
                if request.len() < 2 {
                    return vec![SID_NEGATIVE_RESPONSE, sid, 0x13];
                }
                let session = request[1];
                self.active_session = session;
                // Positive response with P2 (50ms) and P2* (5000ms) timings
                vec![
                    sid + 0x40,
                    session,
                    0x00, 0x32, // P2 = 50ms
                    0x01, 0xF4, // P2* = 5000ms
                ]
            }
            SID_TESTER_PRESENT => {
                let subfunction = if request.len() > 1 { request[1] } else { 0x00 };
                vec![sid + 0x40, subfunction & 0x7F]
            }
            SID_SECURITY_ACCESS => {
                if request.len() < 2 {
                    return vec![SID_NEGATIVE_RESPONSE, sid, 0x13];
                }
                let subfunction = request[1];
                if subfunction == 0x01 || subfunction == 0x03 {
                    // Request Seed
                    let mut resp = vec![sid + 0x40, subfunction];
                    resp.extend_from_slice(&self.seed);
                    resp
                } else if subfunction == 0x02 || subfunction == 0x04 {
                    // Send Key
                    let expected_key: Vec<u8> = self.seed.iter().map(|b| b ^ 0xAA).collect();
                    let vag_login = [0x00, 0x00, 0x4E, 0x87]; // 20103
                    let is_match = (request.len() >= 6 && &request[2..6] == &expected_key[..])
                        || (request.len() >= 6 && &request[2..6] == &vag_login[..]);

                    if is_match {
                        self.security_unlocked = true;
                        vec![sid + 0x40, subfunction]
                    } else {
                        vec![SID_NEGATIVE_RESPONSE, sid, 0x35] // InvalidKey
                    }
                } else {
                    vec![SID_NEGATIVE_RESPONSE, sid, 0x12] // SubFunctionNotSupported
                }
            }
            SID_READ_DATA_BY_IDENTIFIER => {
                if request.len() < 3 {
                    return vec![SID_NEGATIVE_RESPONSE, sid, 0x13];
                }
                let did = ((request[1] as u16) << 8) | (request[2] as u16);
                if let Some(val) = self.dids.get(&did) {
                    let mut resp = vec![sid + 0x40, request[1], request[2]];
                    resp.extend_from_slice(val);
                    resp
                } else {
                    vec![SID_NEGATIVE_RESPONSE, sid, 0x31] // RequestOutOfRange
                }
            }
            SID_WRITE_DATA_BY_IDENTIFIER => {
                if request.len() < 3 {
                    return vec![SID_NEGATIVE_RESPONSE, sid, 0x13];
                }
                let did = ((request[1] as u16) << 8) | (request[2] as u16);
                let payload = &request[3..];
                self.dids.insert(did, payload.to_vec());

                // Inspect specific adaptation modifications
                if did == DID_ADAPTATION_CHANNEL_15 && payload.len() >= 2 {
                    self.svm_03276_resolved = true;
                } else if did == DID_GREEN_MENU_ENABLE || did == DID_CODING {
                    // Green Menu unlock recorded
                } else if did == DID_CAR_MENU_CONFIG && payload.len() >= 2 {
                    let val = u16::from_be_bytes([payload[0], payload[1]]);
                    if val == self.initial_car_menu_setting {
                        self.svm_03175_resolved = true;
                    }
                }

                vec![sid + 0x40, request[1], request[2]]
            }
            SID_CLEAR_DIAGNOSTIC_INFORMATION => {
                self.active_dtcs.clear();
                vec![sid + 0x40]
            }
            _ => vec![SID_NEGATIVE_RESPONSE, sid, 0x11], // ServiceNotSupported
        }
    }
}

impl CanAdapter for LoopbackSimulator {
    fn send(&mut self, frame: &CanFrame) -> Result<(), DiagnosticsError> {
        if frame.id != MMI_REQ_ID || frame.data.is_empty() {
            return Ok(());
        }

        let pci_type = frame.data[0] & 0xF0;

        match pci_type {
            ISOTP_SF => {
                let len = (frame.data[0] & 0x0F) as usize;
                if len > 0 && len < frame.data.len() {
                    let req_payload = frame.data[1..1 + len].to_vec();
                    let resp = self.process_uds_request(&req_payload);
                    self.queue_uds_response(&resp);
                }
            }
            ISOTP_FF => {
                if frame.data.len() >= 8 {
                    let total_len = (((frame.data[0] & 0x0F) as usize) << 8) | (frame.data[1] as usize);
                    self.rx_in_progress = true;
                    self.rx_expected_len = total_len;
                    self.rx_buffer.clear();
                    self.rx_buffer.extend_from_slice(&frame.data[2..8]);
                    self.rx_expected_sn = 1;

                    // Send Flow Control Continue-To-Send back to client
                    let fc_frame = CanFrame::new_standard(MMI_RESP_ID, &[ISOTP_FC | FC_CTS, 0x00, 0x00])
                        .with_padding(0xAA);
                    self.rx_queue.push_back(fc_frame);
                }
            }
            ISOTP_CF => {
                if self.rx_in_progress {
                    let sn = frame.data[0] & 0x0F;
                    if sn == self.rx_expected_sn {
                        self.rx_expected_sn = (self.rx_expected_sn + 1) & 0x0F;
                        let needed = self.rx_expected_len - self.rx_buffer.len();
                        let available = frame.data.len() - 1;
                        let take = needed.min(available);
                        self.rx_buffer.extend_from_slice(&frame.data[1..1 + take]);

                        if self.rx_buffer.len() >= self.rx_expected_len {
                            self.rx_in_progress = false;
                            let req = std::mem::take(&mut self.rx_buffer);
                            let resp = self.process_uds_request(&req);
                            self.queue_uds_response(&resp);
                        }
                    }
                }
            }
            ISOTP_FC => {
                // Client sent Flow Control frame: release pending Consecutive Frames
                while let Some(cf) = self.pending_cf_frames.pop_front() {
                    self.rx_queue.push_back(cf);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn receive(&mut self, _timeout: Duration) -> Result<CanFrame, DiagnosticsError> {
        self.rx_queue
            .pop_front()
            .ok_or(DiagnosticsError::Timeout)
    }

    fn name(&self) -> &str {
        "LoopbackSimulator"
    }
}
