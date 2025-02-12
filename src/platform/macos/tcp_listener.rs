use std::ffi::{c_int, c_void};
use std::mem;
use std::mem::MaybeUninit;
use std::net::{IpAddr, SocketAddr};

use crate::platform::macos::c_libproc::proc_pidfdinfo;
use crate::platform::macos::c_socket_fd_info::CSocketFdInfo;
use crate::platform::macos::proc_pid::ProcPid;
use crate::platform::macos::socket_fd::SocketFd;
use crate::platform::macos::statics::PROC_PID_FD_SOCKET_INFO;

use crate::Protocol;

#[derive(Debug)]
pub(super) struct TcpListener{
    local_addr: SocketAddr,
    protocol: Protocol
}

impl TcpListener {
    pub(super) fn new(addr: IpAddr, port: u16, protocol: Protocol) -> Self {
        TcpListener{
            local_addr: SocketAddr::new(addr, port),
            protocol: protocol,
        }
    }

    pub(super) fn socket_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub(super) fn protocol(&self) -> Protocol {
        self.protocol
    }

    pub(super) fn from_pid_fd(pid: ProcPid, fd: &SocketFd) -> crate::Result<Self> {
        let mut sinfo: MaybeUninit<CSocketFdInfo> = MaybeUninit::uninit();

        let return_code = unsafe {
            proc_pidfdinfo(
                pid.as_c_int(),
                fd.fd(),
                PROC_PID_FD_SOCKET_INFO,
                sinfo.as_mut_ptr().cast::<c_void>(),
                c_int::try_from(mem::size_of::<CSocketFdInfo>())?,
            )
        };

        if return_code < 0 {
            return Err("Failed to get file descriptor information".into());
        }

        let c_socket_fd_info = unsafe { sinfo.assume_init() };
        c_socket_fd_info.to_tcp_listener()
    }
}
