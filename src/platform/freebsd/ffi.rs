use super::socket_info::SocketInfo;
use crate::{Process, Protocol};
use std::ffi::CStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::os::raw::{c_char, c_int};
use std::{io, ptr};

#[repr(C)]
struct CSocketAddress {
    addr: CAddress,
    family: i32,
}

#[repr(C)]
union CAddress {
    ipv4: [u8; 4],
    ipv6: [u8; 16],
}

#[repr(C)]
pub(super) struct CSocketInfo {
    address: CSocketAddress,
    port: u16,
    pub(super) protocol: u32,
}

impl CSocketInfo {
    pub(super) fn to_sockaddr(&self) -> SocketAddr {
        let c_sock_addr = &self.address;
        let ip = if c_sock_addr.family == libc::AF_INET {
            let octets = unsafe { c_sock_addr.addr.ipv4 };
            IpAddr::V4(Ipv4Addr::from_octets(octets))
        } else {
            let octets = unsafe { c_sock_addr.addr.ipv6 };
            IpAddr::V6(Ipv6Addr::from_octets(octets))
        };
        SocketAddr::new(ip, self.port)
    }
}

#[repr(C)]
struct CProcessInfo {
    path: [c_char; libc::PATH_MAX as usize],
    name: [c_char; libc::COMMLEN + 1],
    pid: c_int,
}

unsafe extern "C" {
    fn lsock_tcp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_tcp6(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_udp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_udp6(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn proc_list(list: *mut *mut CProcessInfo, nentries: *mut usize) -> c_int;
    fn proc_sockets(pid: c_int, list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
}

pub(super) fn get_processes() -> io::Result<Vec<Process>> {
    let mut list: *mut CProcessInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { proc_list(&mut list, &mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut processes = Vec::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_processes = std::slice::from_raw_parts(list, nentries);

            for c_process in c_processes.iter() {
                let name = CStr::from_ptr(c_process.name.as_ptr())
                    .to_string_lossy()
                    .into_owned();

                let path = CStr::from_ptr(c_process.path.as_ptr())
                    .to_string_lossy()
                    .into_owned();

                processes.push(Process::new(c_process.pid as u32, name, path));
            }

            libc::free(list as *mut libc::c_void);
        }
    }

    Ok(processes)
}

pub(super) fn get_all_sockets_of_pid(pid: u32) -> Vec<SocketInfo> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { proc_sockets(pid as c_int, &mut list, &mut nentries) } != 0 {
        return Vec::new();
    }

    let mut sockets = Vec::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_sockets = std::slice::from_raw_parts(list, nentries);

            for c_socket in c_sockets {
                sockets.push(SocketInfo::from(c_socket));
            }

            libc::free(list as *mut libc::c_void);
        }
    }

    sockets
}

pub(super) fn get_socket_by_port_of_pid(
    pid: u32,
    port: u16,
    protocol: Protocol,
) -> Option<SocketInfo> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { proc_sockets(pid as c_int, &mut list, &mut nentries) } != 0 {
        return None;
    }

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_sockets = std::slice::from_raw_parts(list, nentries);

            for c_socket in c_sockets {
                let socket_info = SocketInfo::from(c_socket);
                if socket_info.address.port() == port && socket_info.protocol == protocol {
                    libc::free(list as *mut libc::c_void);
                    return Some(socket_info);
                }
            }

            libc::free(list as *mut libc::c_void);
        }
    }

    None
}
