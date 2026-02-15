use crate::Protocol;
use libc::{KI_MAXCOMLEN, pid_t};
use std::{
    collections::HashSet,
    ffi::CStr,
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    os::raw::{c_char, c_int},
    ptr,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct SocketInfo {
    pub(super) address: SocketAddr,
    pub(super) protocol: Protocol,
}

#[repr(C)]
union CAddress {
    ipv4: [u8; 4],
    ipv6: [u8; 16],
}

#[repr(C)]
struct CSocketAddress {
    addr: CAddress,
    family: i32,
}

impl CSocketInfo {
    fn to_socket_info(&self) -> SocketInfo {
        let c_sock_addr = &self.address;
        let ip = if c_sock_addr.family == libc::AF_INET {
            let octets = unsafe { c_sock_addr.addr.ipv4 };
            IpAddr::V4(Ipv4Addr::from(octets))
        } else {
            let octets = unsafe { c_sock_addr.addr.ipv6 };
            IpAddr::V6(Ipv6Addr::from(octets))
        };

        SocketInfo {
            address: SocketAddr::new(ip, self.port),
            protocol: match self.protocol {
                libc::IPPROTO_TCP => Protocol::TCP,
                _ => Protocol::UDP,
            },
        }
    }
}

#[repr(C)]
struct CSocketInfo {
    address: CSocketAddress,
    protocol: i32,
    port: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(super) struct ProcessInfo {
    pub(super) pid: i32,
    pub(super) name: String,
}

#[repr(C)]
struct CProcessInfo {
    pub name: [c_char; KI_MAXCOMLEN as usize],
    pub pid: pid_t,
}

impl From<&CProcessInfo> for ProcessInfo {
    fn from(c_info: &CProcessInfo) -> Self {
        let name = unsafe {
            CStr::from_ptr(c_info.name.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        ProcessInfo {
            pid: c_info.pid,
            name,
        }
    }
}

unsafe extern "C" {
    fn proc_all(list: *mut *mut CProcessInfo, nentries: *mut usize) -> c_int;
    fn socks_by_pid(pid: libc::pid_t, list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
}

pub(super) fn get_all_processes() -> io::Result<HashSet<ProcessInfo>> {
    let mut list: *mut CProcessInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { proc_all(&raw mut list, &raw mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut processes = HashSet::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_processes = std::slice::from_raw_parts(list, nentries);

            for c_process in c_processes {
                processes.insert(c_process.into());
            }

            libc::free(list.cast::<libc::c_void>());
        }
    }

    Ok(processes)
}

pub(super) fn get_sockets(pid: i32) -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { socks_by_pid(pid, &raw mut list, &raw mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut sockets = Vec::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_sockets = std::slice::from_raw_parts(list, nentries);

            for c_socket in c_sockets {
                sockets.push(c_socket.to_socket_info());
            }

            libc::free(list.cast::<libc::c_void>());
        }
    }

    Ok(sockets)
}
