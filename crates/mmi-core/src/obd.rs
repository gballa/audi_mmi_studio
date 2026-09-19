//! OBD-II & CAN-Bus Diagnostic Bridge Engine
//!
//! Provides ELM327 / STN / UDS diagnostic protocol communication for Audi MMI 3G/3G+,
//! supporting live telemetry capture, automated SVM Error 03276 resolution (Channel 15 XOR 51666),
//! Green Engineering Menu (GEM) activation (Channel 6 = 1), and DTC clearing.

use serde::{Deserialize, Serialize};

/// Standard VAG Information Electronics Module Address (Module 5F).
pub const MODULE_5F_CAN_TX_ID: u32 = 0x714;
pub const MODULE_5F_CAN_RX_ID: u32 = 0x77E;

/// VAG SVM Channel 15 XOR Cipher constant (51666 / 0xC9D2).
pub const SVM_CHANNEL_15_XOR_CIPHER: u32 = 51666;

/// Live vehicle telemetry streamed over OBD-II / CAN-Bus.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleTelemetry {
    pub rpm: u16,
    pub speed_kmh: u8,
    pub coolant_temp_c: i16,
    pub control_module_voltage: f32,
    pub ambient_temp_c: i16,
    pub active_drive_select: String,
    pub timestamp_epoch_ms: u64,
}

impl Default for VehicleTelemetry {
    fn default() -> Self {
        Self {
            rpm: 850,
            speed_kmh: 0,
            coolant_temp_c: 90,
            control_module_voltage: 13.8,
            ambient_temp_c: 21,
            active_drive_select: "individual".to_string(),
            timestamp_epoch_ms: 1773940000000,
        }
    }
}

/// Status of an SVM adaptation resolution procedure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SvmResolutionReport {
    pub module_address: String,
    pub adaptation_channel: u8,
    pub initial_challenge_value: u32,
    pub computed_response_value: u32,
    pub write_verified: bool,
    pub dtc_03276_cleared: bool,
    pub audit_timestamp_ms: u64,
}

/// Status of Green Engineering Menu (GEM) activation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GemActivationReport {
    pub module_address: String,
    pub adaptation_channel: u8,
    pub previous_value: u8,
    pub new_value: u8,
    pub gem_unlocked: bool,
    pub reboot_required: bool,
}

/// Complete diagnostic scan summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticSessionReport {
    pub port: String,
    pub baud_rate: u32,
    pub protocol: String,
    pub connected: bool,
    pub telemetry: VehicleTelemetry,
    pub svm_report: Option<SvmResolutionReport>,
    pub gem_report: Option<GemActivationReport>,
    pub dtcs_cleared: usize,
}

/// Computes the exact response for SVM Fault Code 03276 given the Channel 15 challenge.
///
/// Formula: `response = challenge ^ 51666` (or `0xC9D2`).
pub fn solve_svm_03276(channel_15_challenge: u32) -> u32 {
    channel_15_challenge ^ SVM_CHANNEL_15_XOR_CIPHER
}

/// Decodes standard OBD-II PID 010C (Engine RPM).
/// Raw bytes A and B -> `((A * 256) + B) / 4`.
pub fn decode_obd_rpm(a: u8, b: u8) -> u16 {
    (((a as u32 * 256) + (b as u32)) / 4) as u16
}

/// Decodes standard OBD-II PID 010D (Vehicle Speed in km/h).
/// Raw byte A -> `A`.
pub fn decode_obd_speed(a: u8) -> u8 {
    a
}

/// Decodes standard OBD-II PID 0105 (Coolant Temperature in °C).
/// Raw byte A -> `A - 40`.
pub fn decode_obd_coolant_temp(a: u8) -> i16 {
    (a as i16) - 40
}

/// Decodes standard OBD-II PID 0142 (Control Module Voltage).
/// Raw bytes A and B -> `((A * 256) + B) / 1000.0`.
pub fn decode_obd_voltage(a: u8, b: u8) -> f32 {
    let millivolts = (a as u32 * 256) + (b as u32);
    (millivolts as f32) / 1000.0
}

/// High-fidelity Virtual ELM327 / UDS Diagnostic Simulator.
///
/// Allows dry-running vehicle scans, SVM 03276 fixes, and GEM activation
/// without requiring a physical car or hardware dongle.
pub struct VirtualObdBridge {
    connected: bool,
    channel_15_challenge: u32,
    channel_6_gem: u8,
    dtc_03276_present: bool,
}

impl Default for VirtualObdBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualObdBridge {
    pub fn new() -> Self {
        Self {
            connected: true,
            channel_15_challenge: 24581, // Typical genuine VAG MMI 3G+ challenge
            channel_6_gem: 0,            // Locked by default
            dtc_03276_present: true,
        }
    }

    /// Read live telemetry snapshot.
    pub fn read_telemetry(&self) -> VehicleTelemetry {
        VehicleTelemetry {
            rpm: 820,
            speed_kmh: 0,
            coolant_temp_c: 90,
            control_module_voltage: 13.92,
            ambient_temp_c: 22,
            active_drive_select: "dynamic".to_string(),
            timestamp_epoch_ms: 1773941000000,
        }
    }

    /// Executes the full automated SVM Error 03276 resolution on Module 5F Channel 15.
    pub fn resolve_svm_error(&mut self) -> SvmResolutionReport {
        let challenge = self.channel_15_challenge;
        let response = solve_svm_03276(challenge);

        // Write response back to simulated adaptation channel
        self.channel_15_challenge = response;
        self.dtc_03276_present = false;

        SvmResolutionReport {
            module_address: "5F (Information Electr.)".to_string(),
            adaptation_channel: 15,
            initial_challenge_value: challenge,
            computed_response_value: response,
            write_verified: true,
            dtc_03276_cleared: true,
            audit_timestamp_ms: 1773941000000,
        }
    }

    /// Enables the hidden Green Engineering Menu (GEM) via Module 5F Channel 6.
    pub fn enable_green_menu(&mut self) -> GemActivationReport {
        let prev = self.channel_6_gem;
        self.channel_6_gem = 1;

        GemActivationReport {
            module_address: "5F (Information Electr.)".to_string(),
            adaptation_channel: 6,
            previous_value: prev,
            new_value: 1,
            gem_unlocked: true,
            reboot_required: true,
        }
    }

    /// Clears all DTC fault codes on Module 5F.
    pub fn clear_dtcs(&mut self) -> usize {
        let count = if self.dtc_03276_present { 1 } else { 0 };
        self.dtc_03276_present = false;
        count
    }

    /// Runs a full diagnostic routine returning a unified session report.
    pub fn run_session(
        &mut self,
        port: &str,
        baud: u32,
        solve_svm: bool,
        enable_gem: bool,
    ) -> DiagnosticSessionReport {
        let telemetry = self.read_telemetry();
        let svm_report = if solve_svm {
            Some(self.resolve_svm_error())
        } else {
            None
        };
        let gem_report = if enable_gem {
            Some(self.enable_green_menu())
        } else {
            None
        };
        let dtcs_cleared = self.clear_dtcs();

        DiagnosticSessionReport {
            port: port.to_string(),
            baud_rate: baud,
            protocol: "ISO 15765-4 (CAN 11-bit 500kbps) / ISO 14229 (UDS)".to_string(),
            connected: self.connected,
            telemetry,
            svm_report,
            gem_report,
            dtcs_cleared,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svm_03276_xor_calculation() {
        let challenge = 24581;
        let response = solve_svm_03276(challenge);
        assert_eq!(response, 24581 ^ 51666);
        assert_eq!(solve_svm_03276(response), challenge);
    }

    #[test]
    fn test_obd_standard_pid_decoding() {
        // RPM: 0x0F 0xA0 -> ((15 * 256) + 160) / 4 = 4000 / 4 = 1000 RPM
        assert_eq!(decode_obd_rpm(0x0F, 0xA0), 1000);

        // Speed: 120 km/h
        assert_eq!(decode_obd_speed(120), 120);

        // Coolant: 0x82 (130) -> 130 - 40 = 90°C
        assert_eq!(decode_obd_coolant_temp(130), 90);

        // Voltage: 0x36 0x1A (13850 mV) -> 13.85 V
        let v = decode_obd_voltage(0x36, 0x1A);
        assert!((v - 13.85).abs() < 0.01);
    }

    #[test]
    fn test_virtual_obd_bridge_session() {
        let mut bridge = VirtualObdBridge::new();
        let report = bridge.run_session("/dev/tty.usbserial-OBD2", 115200, true, true);

        assert!(report.connected);
        assert_eq!(report.telemetry.coolant_temp_c, 90);

        let svm = report.svm_report.expect("SVM report should be present");
        assert!(svm.write_verified);
        assert!(svm.dtc_03276_cleared);

        let gem = report.gem_report.expect("GEM report should be present");
        assert!(gem.gem_unlocked);
        assert_eq!(gem.new_value, 1);
    }
}
