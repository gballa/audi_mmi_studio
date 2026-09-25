use std::io::{Read, Write};
use std::time::{Duration, Instant};
use crate::can::{CanAdapter, CanFrame};
use crate::error::DiagnosticsError;

/// Serial ELM327 / STN1170 OBD-II adapter implementation
pub struct SerialElmAdapter {
    pub port_name: String,
    pub baud_rate: u32,
    pub connected: bool,
    pub tx_id: u32,
    pub rx_id: u32,
    #[cfg(unix)]
    file: Option<std::fs::File>,
    rx_buffer: String,
}

impl SerialElmAdapter {
    pub fn new(port_name: &str, baud_rate: u32) -> Self {
        Self {
            port_name: port_name.to_string(),
            baud_rate,
            connected: false,
            tx_id: 0x714,
            rx_id: 0x77E,
            #[cfg(unix)]
            file: None,
            rx_buffer: String::new(),
        }
    }

    /// Formats a CanFrame into an ELM327 transmission hex string (e.g. "021003\r")
    pub fn format_frame_tx(frame: &CanFrame) -> String {
        let mut s = String::with_capacity(frame.data.len() * 2 + 1);
        for byte in &frame.data {
            s.push_str(&format!("{:02X}", byte));
        }
        s.push('\r');
        s
    }

    /// Parses raw ELM327 response strings (e.g. "06 50 03 00 32 01 F4\r\r>" or "77E 03 7F 22 31\r\r>") into CanFrames
    pub fn parse_elm_response(raw_output: &str, default_rx_id: u32) -> Result<Vec<CanFrame>, DiagnosticsError> {
        let mut frames = Vec::new();

        for line in raw_output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == ">" || trimmed == "OK" {
                continue;
            }

            if trimmed.contains("CAN ERROR") || trimmed.contains("ERR") {
                return Err(DiagnosticsError::Can(format!("ELM327 hardware error: {}", trimmed)));
            }

            if trimmed.contains("BUFFER FULL") {
                return Err(DiagnosticsError::IsoTp("ELM327 buffer overflow".into()));
            }

            if trimmed.contains("NO DATA") || trimmed.contains("SEARCHING...") {
                continue;
            }

            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            let mut can_id = default_rx_id;
            let mut byte_tokens = &tokens[..];

            // If first token is 3 hex chars (e.g. "77E"), it is an 11-bit CAN ID
            if tokens[0].len() == 3 && tokens[0].chars().all(|c| c.is_ascii_hexdigit()) {
                if let Ok(id) = u32::from_str_radix(tokens[0], 16) {
                    can_id = id;
                    byte_tokens = &tokens[1..];
                }
            } else if tokens[0].len() == 8 && tokens[0].chars().all(|c| c.is_ascii_hexdigit()) && tokens.len() > 1 {
                // 29-bit CAN ID
                if let Ok(id) = u32::from_str_radix(tokens[0], 16) {
                    can_id = id;
                    byte_tokens = &tokens[1..];
                }
            }

            let mut data = Vec::new();
            for tok in byte_tokens {
                if tok.len() == 2 && tok.chars().all(|c| c.is_ascii_hexdigit()) {
                    if let Ok(b) = u8::from_str_radix(tok, 16) {
                        data.push(b);
                    }
                }
            }

            if data.is_empty() {
                // Try decoding contiguous hex string
                let hex_clean: String = trimmed.chars().filter(|c| c.is_ascii_hexdigit()).collect();
                if hex_clean.len() >= 2 && hex_clean.len() % 2 == 0 {
                    if let Ok(b) = hex::decode(&hex_clean) {
                        data = b;
                    }
                }
            }

            if !data.is_empty() {
                let frame = if data.len() > 8 {
                    CanFrame::new_standard(can_id, &data[..8])
                } else {
                    CanFrame::new_standard(can_id, &data)
                };
                frames.push(frame);
            }
        }

        Ok(frames)
    }

    /// Connects to the physical serial port and executes standard ELM327 AT initialization
    pub fn connect(&mut self) -> Result<(), DiagnosticsError> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            use std::os::unix::io::AsRawFd;

            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(libc::O_NONBLOCK | libc::O_NOCTTY)
                .open(&self.port_name)
                .map_err(|e| DiagnosticsError::Adapter(format!("Cannot open port '{}': {}", self.port_name, e)))?;

            let fd = file.as_raw_fd();
            unsafe {
                let mut termios: libc::termios = std::mem::zeroed();
                if libc::tcgetattr(fd, &mut termios) == 0 {
                    // Raw mode 8N1
                    termios.c_cflag |= libc::CS8 | libc::CLOCAL | libc::CREAD;
                    termios.c_cflag &= !(libc::PARENB | libc::CSTOPB);
                    termios.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ECHOE | libc::ISIG);
                    termios.c_iflag &= !(libc::IXON | libc::IXOFF | libc::IXANY | libc::ICRNL | libc::INLCR);
                    termios.c_oflag &= !libc::OPOST;

                    // Baud rate configuration
                    let speed = match self.baud_rate {
                        9600 => libc::B9600,
                        19200 => libc::B19200,
                        38400 => libc::B38400,
                        57600 => libc::B57600,
                        115200 => libc::B115200,
                        230400 => libc::B230400,
                        _ => libc::B115200,
                    };
                    libc::cfsetispeed(&mut termios, speed);
                    libc::cfsetospeed(&mut termios, speed);
                    libc::tcsetattr(fd, libc::TCSANOW, &termios);
                }
            }

            self.file = Some(file);
            self.connected = true;

            // Transmit ELM327 initialization sequence
            let init_cmds = [
                "AT Z\r",      // Reset device
                "AT E0\r",     // Echo off
                "AT L0\r",     // Linefeeds off
                "AT S0\r",     // Spaces off
                "AT SP 6\r",   // Protocol ISO 15765-4 (CAN 11/500)
                "AT CRA 77E\r",// Receive filter
                "AT SH 714\r", // Header transmit ID
                "AT AT 1\r",   // Adaptive timing
            ];

            for cmd in &init_cmds {
                let _ = self.write_raw(cmd.as_bytes());
                std::thread::sleep(Duration::from_millis(15));
            }

            Ok(())
        }
        #[cfg(not(unix))]
        {
            self.connected = true;
            Ok(())
        }
    }

    #[cfg(unix)]
    fn write_raw(&mut self, data: &[u8]) -> Result<(), DiagnosticsError> {
        if let Some(ref mut file) = self.file {
            file.write_all(data).map_err(DiagnosticsError::Io)?;
            file.flush().map_err(DiagnosticsError::Io)?;
            Ok(())
        } else {
            Err(DiagnosticsError::Adapter("Serial device not opened".into()))
        }
    }

    #[cfg(not(unix))]
    fn write_raw(&mut self, _data: &[u8]) -> Result<(), DiagnosticsError> {
        Ok(())
    }
}

impl CanAdapter for SerialElmAdapter {
    fn send(&mut self, frame: &CanFrame) -> Result<(), DiagnosticsError> {
        if !self.connected {
            return Err(DiagnosticsError::Adapter("Serial device not connected".into()));
        }

        let cmd = Self::format_frame_tx(frame);
        self.write_raw(cmd.as_bytes())
    }

    fn receive(&mut self, timeout: Duration) -> Result<CanFrame, DiagnosticsError> {
        if !self.connected {
            return Err(DiagnosticsError::Adapter("Serial device not connected".into()));
        }

        let start = Instant::now();
        let mut buf = [0u8; 128];

        loop {
            if start.elapsed() > timeout {
                return Err(DiagnosticsError::Timeout);
            }

            #[cfg(unix)]
            if let Some(ref mut file) = self.file {
                match file.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let chunk = String::from_utf8_lossy(&buf[..n]);
                        self.rx_buffer.push_str(&chunk);

                        if self.rx_buffer.contains('>') {
                            let complete = std::mem::take(&mut self.rx_buffer);
                            let frames = Self::parse_elm_response(&complete, self.rx_id)?;
                            if let Some(first) = frames.into_iter().next() {
                                return Ok(first);
                            }
                        }
                    }
                    Ok(_) => {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(e) => return Err(DiagnosticsError::Io(e)),
                }
            }

            #[cfg(not(unix))]
            {
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }

    fn name(&self) -> &str {
        "SerialElmAdapter"
    }
}
