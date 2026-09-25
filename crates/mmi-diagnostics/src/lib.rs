//! mmi-diagnostics: Automotive UDS (ISO 14229-1) & ISO-TP (ISO 15765-2) Diagnostic Communication Engine
//! for Audi MMI 3G / 3G+ (HN+) platforms.

pub mod error;
pub mod can;
pub mod isotp;
pub mod uds;
pub mod adapters;
pub mod svm;

pub use error::{nrc_description, DiagnosticsError};
pub use can::{CanAdapter, CanFrame};
pub use isotp::{
    decode_st_min, encode_st_min, IsoTpChannel, IsoTpConfig, FC_CTS, FC_OVERFLOW, FC_WAIT,
    ISOTP_CF, ISOTP_FC, ISOTP_FF, ISOTP_SF, MAX_ISOTP_PAYLOAD_LEN,
};
pub use uds::{
    UdsClient, DID_ADAPTATION_CHANNEL_15, DID_CAR_MENU_CONFIG, DID_CODING, DID_ECU_PART_NUMBER,
    DID_GREEN_MENU_ENABLE, DID_SOFTWARE_VERSION, DID_SUPPLY_VOLTAGE, DID_SYSTEM_NAME, DID_VIN,
    NRC_RESPONSE_PENDING, SESSION_DEFAULT, SESSION_EXTENDED, SESSION_PROGRAMMING, SESSION_SAFETY,
    SID_CLEAR_DIAGNOSTIC_INFORMATION, SID_DIAGNOSTIC_SESSION_CONTROL, SID_NEGATIVE_RESPONSE,
    SID_READ_DATA_BY_IDENTIFIER, SID_READ_DTC_INFORMATION, SID_SECURITY_ACCESS,
    SID_TESTER_PRESENT, SID_WRITE_DATA_BY_IDENTIFIER,
};
pub use adapters::{LoopbackSimulator, SerialElmAdapter, SocketCanAdapter};
pub use svm::{
    Svm03175Report, SvmResolutionResult, SvmSolver, SVM_CHANNEL_15_XOR_CIPHER,
    SVM_CHANNEL_15_XOR_KEY,
};
