use crate::Protocol;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::os::raw::c_int;
use std::{io, ptr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct SocketInfo {
    pub(super) address: SocketAddr,
    pub(super) protocol: Protocol,
    pub(super) kvaddr: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct SocketFile {
    pub(super) kvaddr: usize,
    pub(super) pid: u32,
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

#[repr(C)]
struct CSocketInfo {
    address: CSocketAddress,
    kvaddr: usize,
    protocol: i32,
    port: u16,
}

#[repr(C)]
struct CSocketFile {
    kvaddr: usize,
    pid: libc::pid_t,
}

impl CSocketInfo {
    fn to_socket_info(&self) -> SocketInfo {
        let c_sock_addr = &self.address;
        let ip = if c_sock_addr.family == libc::AF_INET {
            let octets = unsafe { c_sock_addr.addr.ipv4 };
            IpAddr::V4(Ipv4Addr::from_octets(octets))
        } else {
            let octets = unsafe { c_sock_addr.addr.ipv6 };
            IpAddr::V6(Ipv6Addr::from_octets(octets))
        };

        SocketInfo {
            kvaddr: self.kvaddr,
            address: SocketAddr::new(ip, self.port),
            protocol: match self.protocol {
                libc::IPPROTO_TCP => Protocol::TCP,
                _ => Protocol::UDP,
            },
        }
    }
}

impl CSocketFile {
    fn to_socket_file(&self) -> SocketFile {
        SocketFile {
            kvaddr: self.kvaddr,
            pid: self.pid as u32,
        }
    }
}

unsafe extern "C" {
    fn lsock_tcp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_udp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_files(list: *mut *mut CSocketFile, nentries: *mut usize) -> c_int;

}

pub(super) fn get_tcp_sockets() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_tcp(&mut list, &mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut sockets = Vec::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_sockets = std::slice::from_raw_parts(list, nentries);

            for c_socket in c_sockets {
                sockets.push(c_socket.to_socket_info());
            }

            libc::free(list as *mut libc::c_void);
        }
    }

    Ok(sockets)
}

pub(super) fn get_udp_sockets() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_udp(&mut list, &mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut sockets = Vec::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_sockets = std::slice::from_raw_parts(list, nentries);

            for c_socket in c_sockets {
                sockets.push(c_socket.to_socket_info());
            }

            libc::free(list as *mut libc::c_void);
        }
    }

    Ok(sockets)
}

pub(super) fn get_kvaddr_to_pid_table() -> io::Result<HashMap<usize, i32>> {
    let mut list: *mut CSocketFile = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_files(&mut list, &mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut retval = HashMap::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_files = std::slice::from_raw_parts(list, nentries);

            for c_file in c_files {
                retval.insert(c_file.kvaddr, c_file.pid);
            }

            libc::free(list as *mut libc::c_void);
        }
    }

    Ok(retval)
}
