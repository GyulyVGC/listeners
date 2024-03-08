use crate::platform::macos::c_socket_fd_info::{CSocketFdInfo, InSockinfo};
use crate::platform::macos::libproc::proc_pidfdinfo;
use crate::platform::macos::pid::Pid;
use crate::platform::macos::socket_fd::SocketFd;
use crate::platform::macos::statics::{PROC_PID_FD_SOCKET_INFO, SOCKET_STATE_LISTEN};
use byteorder::{ByteOrder, NetworkEndian};
use std::ffi::{c_int, c_void};
use std::mem;
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

#[derive(Debug)]
pub(super) struct LocalSocket {
    socket_addr: SocketAddr,
}

impl LocalSocket {
    pub(super) fn new(addr: IpAddr, port: u16) -> Self {
        LocalSocket {
            socket_addr: SocketAddr::new(addr, port),
        }
    }

    pub(super) fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }

    pub(super) fn from_pid_fd(pid: Pid, fd: SocketFd) -> crate::Result<Self> {
        let mut sinfo: MaybeUninit<CSocketFdInfo> = MaybeUninit::uninit();

        let return_code = unsafe {
            proc_pidfdinfo(
                pid.as_c_int(),
                fd.fd(),
                PROC_PID_FD_SOCKET_INFO,
                sinfo.as_mut_ptr().cast::<c_void>(),
                mem::size_of::<CSocketFdInfo>() as i32,
            )
        };

        if return_code < 0 {
            return Err("Failed to get file descriptor information".into());
        }

        Self::from_c_socket_fd_info(unsafe { sinfo.assume_init() })
    }

    fn from_c_socket_fd_info(sinfo: CSocketFdInfo) -> crate::Result<LocalSocket> {
        let sock_info = sinfo.psi;
        let family = sock_info.soi_family;

        let tcp_in = unsafe { sock_info.soi_proto.pri_tcp };

        if tcp_in.tcpsi_state != SOCKET_STATE_LISTEN {
            return Err("Socket is not in listen state".into());
        }

        let tcp_sockaddr_in = tcp_in.tcpsi_ini;
        let lport_bytes: [u8; 4] = i32::to_le_bytes(tcp_sockaddr_in.insi_lport);
        let local_address = Self::get_local_addr(family, tcp_sockaddr_in)?;

        let socket_info = LocalSocket::new(local_address, NetworkEndian::read_u16(&lport_bytes));

        Ok(socket_info)
    }

    fn get_local_addr(family: c_int, saddr: InSockinfo) -> crate::Result<IpAddr> {
        match family {
            2 => {
                // AF_INET
                let addr = unsafe { saddr.insi_laddr.ina_46.i46a_addr4.s_addr };
                Ok(IpAddr::V4(Ipv4Addr::from(u32::from_be(addr))))
            }
            30 => {
                // AF_INET6
                let addr = unsafe { &saddr.insi_laddr.ina_6.__u6_addr.__u6_addr8 };
                let mut ipv6_addr = [0_u16; 8];
                NetworkEndian::read_u16_into(addr, &mut ipv6_addr);
                Ok(IpAddr::V6(Ipv6Addr::from(ipv6_addr)))
            }
            _ => Err("Unsupported socket family".into()),
        }
    }
}
