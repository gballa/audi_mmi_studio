use crate::can::CanAdapter;
use crate::error::{nrc_description, DiagnosticsError};
use crate::isotp::{IsoTpChannel, IsoTpConfig};

// UDS Service Identifiers (ISO 14229-1)
pub const SID_DIAGNOSTIC_SESSION_CONTROL: u8 = 0x10;
pub const SID_CLEAR_DIAGNOSTIC_INFORMATION: u8 = 0x14;
pub const SID_READ_DTC_INFORMATION: u8 = 0x19;
pub const SID_READ_DATA_BY_IDENTIFIER: u8 = 0x22;
pub const SID_SECURITY_ACCESS: u8 = 0x27;
pub const SID_WRITE_DATA_BY_IDENTIFIER: u8 = 0x2E;
pub const SID_TESTER_PRESENT: u8 = 0x3E;
pub const SID_NEGATIVE_RESPONSE: u8 = 0x7F;

// Diagnostic Session Types
pub const SESSION_DEFAULT: u8 = 0x01;
pub const SESSION_PROGRAMMING: u8 = 0x02;
pub const SESSION_EXTENDED: u8 = 0x03;
pub const SESSION_SAFETY: u8 = 0x04;

// Common VAG MMI Data Identifiers (DIDs)
pub const DID_ADAPTATION_CHANNEL_15: u16 = 0x0615; // SVM 03276 calculation channel
pub const DID_CODING: u16 = 0x0606;
pub const DID_GREEN_MENU_ENABLE: u16 = 0x0611;
pub const DID_CAR_MENU_CONFIG: u16 = 0x0620; // Channel 32 / Car Menu Operation
pub const DID_ECU_PART_NUMBER: u16 = 0xF187;
pub const DID_SOFTWARE_VERSION: u16 = 0xF189;
pub const DID_VIN: u16 = 0xF190;
pub const DID_SYSTEM_NAME: u16 = 0xF197;
pub const DID_SUPPLY_VOLTAGE: u16 = 0xF113;

pub const NRC_RESPONSE_PENDING: u8 = 0x78;

pub struct UdsClient<'a> {
    channel: IsoTpChannel,
    adapter: &'a mut dyn CanAdapter,
}

impl<'a> UdsClient<'a> {
    pub fn new(adapter: &'a mut dyn CanAdapter, config: IsoTpConfig) -> Self {
        Self {
            channel: IsoTpChannel::new(config),
            adapter,
        }
    }

    /// Sends a raw UDS request and parses the positive response payload.
    /// Handles NRC 0x78 (Response Pending) by awaiting the subsequent completion response.
    pub fn send_request(&mut self, request: &[u8]) -> Result<Vec<u8>, DiagnosticsError> {
        if request.is_empty() {
            return Err(DiagnosticsError::InvalidResponse("Empty UDS request".into()));
        }
        let requested_sid = request[0];

        self.channel.send(self.adapter, request)?;

        loop {
            let response = self.channel.receive(self.adapter)?;

            if response.is_empty() {
                return Err(DiagnosticsError::InvalidResponse("Empty UDS response".into()));
            }

            // Check for Negative Response (0x7F <SID> <NRC>)
            if response[0] == SID_NEGATIVE_RESPONSE {
                if response.len() < 3 {
                    return Err(DiagnosticsError::InvalidResponse(
                        "Truncated negative response".into(),
                    ));
                }
                let rejected_sid = response[1];
                let nrc = response[2];

                if nrc == NRC_RESPONSE_PENDING {
                    // ResponsePending (0x78): ECU is executing a long operation, await real response
                    continue;
                }

                return Err(DiagnosticsError::NegativeResponse {
                    service_id: rejected_sid,
                    nrc,
                    description: nrc_description(nrc).to_string(),
                });
            }

            // Positive response SID is request SID + 0x40
            let expected_pos_sid = requested_sid.wrapping_add(0x40);
            if response[0] != expected_pos_sid {
                return Err(DiagnosticsError::InvalidResponse(format!(
                    "Unexpected response SID: expected 0x{:02X}, received 0x{:02X}",
                    expected_pos_sid, response[0]
                )));
            }

            return Ok(response[1..].to_vec());
        }
    }

    /// DiagnosticSessionControl (0x10)
    pub fn session_control(&mut self, session_type: u8) -> Result<Vec<u8>, DiagnosticsError> {
        let req = [SID_DIAGNOSTIC_SESSION_CONTROL, session_type];
        self.send_request(&req)
    }

    /// TesterPresent (0x3E)
    pub fn tester_present(&mut self, suppress_pos_rsp: bool) -> Result<(), DiagnosticsError> {
        let subfunction = if suppress_pos_rsp { 0x80 } else { 0x00 };
        let req = [SID_TESTER_PRESENT, subfunction];
        if suppress_pos_rsp {
            // Transmit frame without awaiting response
            self.channel.send(self.adapter, &req)?;
            Ok(())
        } else {
            let _ = self.send_request(&req)?;
            Ok(())
        }
    }

    /// ReadDataByIdentifier (0x22)
    pub fn read_did(&mut self, did: u16) -> Result<Vec<u8>, DiagnosticsError> {
        let req = [
            SID_READ_DATA_BY_IDENTIFIER,
            ((did >> 8) & 0xFF) as u8,
            (did & 0xFF) as u8,
        ];
        let resp = self.send_request(&req)?;
        if resp.len() < 2 {
            return Err(DiagnosticsError::InvalidResponse(
                "ReadDID response missing DID echo".into(),
            ));
        }
        let echoed_did = ((resp[0] as u16) << 8) | (resp[1] as u16);
        if echoed_did != did {
            return Err(DiagnosticsError::InvalidResponse(format!(
                "Echoed DID mismatch: requested 0x{:04X}, received 0x{:04X}",
                did, echoed_did
            )));
        }
        Ok(resp[2..].to_vec())
    }

    /// WriteDataByIdentifier (0x2E)
    pub fn write_did(&mut self, did: u16, data: &[u8]) -> Result<(), DiagnosticsError> {
        let mut req = Vec::with_capacity(3 + data.len());
        req.push(SID_WRITE_DATA_BY_IDENTIFIER);
        req.push(((did >> 8) & 0xFF) as u8);
        req.push((did & 0xFF) as u8);
        req.extend_from_slice(data);

        let resp = self.send_request(&req)?;
        if resp.len() < 2 {
            return Err(DiagnosticsError::InvalidResponse(
                "WriteDID response missing DID echo".into(),
            ));
        }
        let echoed_did = ((resp[0] as u16) << 8) | (resp[1] as u16);
        if echoed_did != did {
            return Err(DiagnosticsError::InvalidResponse(format!(
                "Echoed DID mismatch: wrote 0x{:04X}, received 0x{:04X}",
                did, echoed_did
            )));
        }
        Ok(())
    }

    /// SecurityAccess (0x27) with seed request and key transmission
    pub fn security_access<F>(&mut self, level: u8, key_calculator: F) -> Result<(), DiagnosticsError>
    where
        F: FnOnce(&[u8]) -> Vec<u8>,
    {
        // 1. Request Seed (odd subfunction: level)
        let seed_req = [SID_SECURITY_ACCESS, level];
        let seed_resp = self.send_request(&seed_req)?;
        if seed_resp.is_empty() {
            return Err(DiagnosticsError::InvalidResponse(
                "SecurityAccess response missing subfunction".into(),
            ));
        }
        if seed_resp[0] != level {
            return Err(DiagnosticsError::InvalidResponse(
                "SecurityAccess level mismatch".into(),
            ));
        }
        let seed = &seed_resp[1..];
        if seed.iter().all(|&b| b == 0) {
            // Seed of all zeros indicates security is already unlocked
            return Ok(());
        }

        // 2. Compute and send Key (even subfunction: level + 1)
        let key = key_calculator(seed);
        let mut key_req = Vec::with_capacity(2 + key.len());
        key_req.push(SID_SECURITY_ACCESS);
        key_req.push(level + 1);
        key_req.extend_from_slice(&key);

        let key_resp = self.send_request(&key_req)?;
        if key_resp.is_empty() || key_resp[0] != level + 1 {
            return Err(DiagnosticsError::SecurityAccessDenied);
        }

        Ok(())
    }

    /// Convenience VAG Security Access login (e.g. 20103 / 0x4E87)
    pub fn security_access_login(&mut self, level: u8, login_code: u32) -> Result<(), DiagnosticsError> {
        let code_bytes = login_code.to_be_bytes();
        self.security_access(level, |_seed| code_bytes.to_vec())
    }

    /// ClearDiagnosticInformation (0x14)
    pub fn clear_dtcs(&mut self, dtc_group: u32) -> Result<(), DiagnosticsError> {
        let req = [
            SID_CLEAR_DIAGNOSTIC_INFORMATION,
            ((dtc_group >> 16) & 0xFF) as u8,
            ((dtc_group >> 8) & 0xFF) as u8,
            (dtc_group & 0xFF) as u8,
        ];
        self.send_request(&req)?;
        Ok(())
    }
}
