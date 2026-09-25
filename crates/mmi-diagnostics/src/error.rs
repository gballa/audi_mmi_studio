use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiagnosticsError {
    #[error("CAN communication error: {0}")]
    Can(String),

    #[error("ISO-TP protocol error: {0}")]
    IsoTp(String),

    #[error("Timeout waiting for CAN/ISO-TP response")]
    Timeout,

    #[error("UDS Negative Response: Service 0x{service_id:02X}, NRC 0x{nrc:02X} ({description})")]
    NegativeResponse {
        service_id: u8,
        nrc: u8,
        description: String,
    },

    #[error("Invalid UDS response length or format: {0}")]
    InvalidResponse(String),

    #[error("Security access denied or invalid key")]
    SecurityAccessDenied,

    #[error("Adapter error: {0}")]
    Adapter(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn nrc_description(nrc: u8) -> &'static str {
    match nrc {
        0x10 => "GeneralReject",
        0x11 => "ServiceNotSupported",
        0x12 => "SubFunctionNotSupported",
        0x13 => "IncorrectMessageLengthOrInvalidFormat",
        0x14 => "ResponseTooLong",
        0x21 => "BusyRepeatRequest",
        0x22 => "ConditionsNotCorrect",
        0x24 => "RequestSequenceError",
        0x31 => "RequestOutOfRange",
        0x33 => "SecurityAccessDenied",
        0x35 => "InvalidKey",
        0x36 => "ExceedNumberOfAttempts",
        0x37 => "RequiredTimeDelayNotExpired",
        0x78 => "RequestCorrectlyReceived-ResponsePending",
        0x7E => "SubFunctionNotSupportedInActiveSession",
        0x7F => "ServiceNotSupportedInActiveSession",
        _ => "UnknownNRC",
    }
}
