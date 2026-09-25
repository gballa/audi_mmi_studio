use std::time::Duration;
use crate::can::{CanAdapter, CanFrame};
use crate::error::DiagnosticsError;

/// Linux SocketCAN adapter driver using AF_CAN raw sockets
pub struct SocketCanAdapter {
    pub interface_name: String,
    pub is_open: bool,
    #[cfg(target_os = "linux")]
    sock_fd: Option<i32>,
    pub rx_id: u32,
}

impl SocketCanAdapter {
    pub fn new(interface_name: &str) -> Self {
        Self {
            interface_name: interface_name.to_string(),
            is_open: false,
            #[cfg(target_os = "linux")]
            sock_fd: None,
            rx_id: 0x77E,
        }
    }

    pub fn open(&mut self) -> Result<(), DiagnosticsError> {
        #[cfg(target_os = "linux")]
        {
            use std::ffi::CString;

            unsafe {
                let sock = libc::socket(libc::AF_CAN, libc::SOCK_RAW, 1 /* CAN_RAW */);
                if sock < 0 {
                    return Err(DiagnosticsError::Adapter(format!(
                        "Failed to create AF_CAN raw socket: errno {}",
                        *libc::__errno_location()
                    )));
                }

                // Query network interface index
                let mut ifr: libc::ifreq = std::mem::zeroed();
                let c_name = CString::new(self.interface_name.as_str())
                    .map_err(|_| DiagnosticsError::Adapter("Invalid interface name".into()))?;
                let name_bytes = c_name.as_bytes_with_nul();
                if name_bytes.len() > libc::IFNAMSIZ {
                    libc::close(sock);
                    return Err(DiagnosticsError::Adapter("Interface name too long".into()));
                }
                std::ptr::copy_nonoverlapping(
                    name_bytes.as_ptr() as *const libc::c_char,
                    ifr.ifr_name.as_mut_ptr(),
                    name_bytes.len(),
                );

                if libc::ioctl(sock, libc::SIOCGIFINDEX, &mut ifr) < 0 {
                    libc::close(sock);
                    return Err(DiagnosticsError::Adapter(format!(
                        "Failed to find interface '{}'",
                        self.interface_name
                    )));
                }

                let ifindex = ifr.ifr_ifru.ifru_ivalue;

                // Bind to interface
                #[repr(C)]
                struct SockAddrCan {
                    can_family: libc::sa_family_t,
                    can_ifindex: libc::c_int,
                    can_addr: [u8; 8], // rx_id / tx_id union
                }

                let addr = SockAddrCan {
                    can_family: libc::AF_CAN as libc::sa_family_t,
                    can_ifindex: ifindex,
                    can_addr: [0; 8],
                };

                if libc::bind(
                    sock,
                    &addr as *const SockAddrCan as *const libc::sockaddr,
                    std::mem::size_of::<SockAddrCan>() as libc::socklen_t,
                ) < 0
                {
                    libc::close(sock);
                    return Err(DiagnosticsError::Adapter(format!(
                        "Failed to bind SocketCAN to '{}'",
                        self.interface_name
                    )));
                }

                // Set hardware CAN reception filter
                #[repr(C)]
                struct CanFilter {
                    can_id: u32,
                    can_mask: u32,
                }
                let filter = CanFilter {
                    can_id: self.rx_id,
                    can_mask: 0x7FF,
                };
                libc::setsockopt(
                    sock,
                    1, /* SOL_CAN_RAW */
                    1, /* CAN_RAW_FILTER */
                    &filter as *const CanFilter as *const libc::c_void,
                    std::mem::size_of::<CanFilter>() as libc::socklen_t,
                );

                self.sock_fd = Some(sock);
                self.is_open = true;
                Ok(())
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(DiagnosticsError::PlatformNotSupported(
                "SocketCAN is only supported natively on Linux platforms (AF_CAN); use SerialElmAdapter or LoopbackSimulator on macOS".into(),
            ))
        }
    }

    pub fn close(&mut self) {
        #[cfg(target_os = "linux")]
        if let Some(fd) = self.sock_fd.take() {
            unsafe {
                libc::close(fd);
            }
        }
        self.is_open = false;
    }
}

impl Drop for SocketCanAdapter {
    fn drop(&mut self) {
        self.close();
    }
}

impl CanAdapter for SocketCanAdapter {
    fn send(&mut self, frame: &CanFrame) -> Result<(), DiagnosticsError> {
        if !self.is_open {
            return Err(DiagnosticsError::Adapter("SocketCAN interface not open".into()));
        }

        #[cfg(target_os = "linux")]
        {
            #[repr(C)]
            struct RawCanFrame {
                can_id: u32,
                can_dlc: u8,
                pad: u8,
                res0: u8,
                res1: u8,
                data: [u8; 8],
            }

            if let Some(fd) = self.sock_fd {
                let mut raw = RawCanFrame {
                    can_id: if frame.extended { frame.id | 0x80000000 } else { frame.id },
                    can_dlc: frame.data.len().min(8) as u8,
                    pad: 0,
                    res0: 0,
                    res1: 0,
                    data: [0; 8],
                };
                let copy_len = frame.data.len().min(8);
                raw.data[..copy_len].copy_from_slice(&frame.data[..copy_len]);

                let res = unsafe {
                    libc::write(
                        fd,
                        &raw as *const RawCanFrame as *const libc::c_void,
                        std::mem::size_of::<RawCanFrame>(),
                    )
                };
                if res < 0 {
                    return Err(DiagnosticsError::Can(format!(
                        "SocketCAN write failed: errno {}",
                        unsafe { *libc::__errno_location() }
                    )));
                }
                return Ok(());
            }
            Err(DiagnosticsError::Adapter("SocketCAN descriptor invalid".into()))
        }

        #[cfg(not(target_os = "linux"))]
        {
            let _ = frame;
            Err(DiagnosticsError::PlatformNotSupported(
                "SocketCAN is only supported on Linux".into(),
            ))
        }
    }

    fn receive(&mut self, timeout: Duration) -> Result<CanFrame, DiagnosticsError> {
        if !self.is_open {
            return Err(DiagnosticsError::Adapter("SocketCAN interface not open".into()));
        }

        #[cfg(target_os = "linux")]
        {
            #[repr(C)]
            struct RawCanFrame {
                can_id: u32,
                can_dlc: u8,
                pad: u8,
                res0: u8,
                res1: u8,
                data: [u8; 8],
            }

            if let Some(fd) = self.sock_fd {
                unsafe {
                    let mut pfd = libc::pollfd {
                        fd,
                        events: libc::POLLIN,
                        revents: 0,
                    };
                    let timeout_ms = timeout.as_millis().min(i32::MAX as u128) as libc::c_int;
                    let ret = libc::poll(&mut pfd, 1, timeout_ms);
                    if ret <= 0 {
                        return Err(DiagnosticsError::Timeout);
                    }

                    let mut raw: RawCanFrame = std::mem::zeroed();
                    let n = libc::read(
                        fd,
                        &mut raw as *mut RawCanFrame as *mut libc::c_void,
                        std::mem::size_of::<RawCanFrame>(),
                    );
                    if n <= 0 {
                        return Err(DiagnosticsError::Can("SocketCAN read failed".into()));
                    }

                    let extended = (raw.can_id & 0x80000000) != 0;
                    let id = if extended { raw.can_id & 0x1FFFFFFF } else { raw.can_id & 0x7FF };
                    let dlc = raw.can_dlc.min(8) as usize;
                    let data = raw.data[..dlc].to_vec();

                    return Ok(if extended {
                        CanFrame::new_extended(id, &data)
                    } else {
                        CanFrame::new_standard(id, &data)
                    });
                }
            }
            Err(DiagnosticsError::Adapter("SocketCAN descriptor invalid".into()))
        }

        #[cfg(not(target_os = "linux"))]
        {
            let _ = timeout;
            Err(DiagnosticsError::PlatformNotSupported(
                "SocketCAN is only supported on Linux".into(),
            ))
        }
    }

    fn name(&self) -> &str {
        "SocketCanAdapter"
    }
}
