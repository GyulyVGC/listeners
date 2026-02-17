use crate::Protocol;
use std::{
    collections::{HashMap, hash_map::Entry},
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
    pub(super) kvaddr: u64,
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
            kvaddr: self.kvaddr,
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
    kvaddr: u64,
    protocol: i32,
    port: u16,
}

#[repr(C)]
struct CSocketFile {
    kvaddr: u64,
    pid: libc::pid_t,
}

unsafe extern "C" {
    fn lsock_tcp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_tcp6(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_udp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_udp6(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;

    fn lsock_files(list: *mut *mut CSocketFile, nentries: *mut usize) -> c_int;

    fn proc_name(pid: libc::pid_t) -> *mut c_char;
    fn proc_path(pid: libc::pid_t) -> *mut c_char;
}

pub(super) fn get_tcp_sockets() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_tcp(&raw mut list, &raw mut nentries) } != 0 {
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

pub(super) fn get_tcp6_sockets() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_tcp6(&raw mut list, &raw mut nentries) } != 0 {
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

pub(super) fn get_udp6_sockets() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_udp6(&raw mut list, &raw mut nentries) } != 0 {
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

pub(super) fn get_udp_sockets() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_udp(&raw mut list, &raw mut nentries) } != 0 {
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

pub(super) fn get_kvaddr_to_pid_table() -> io::Result<HashMap<u64, i32>> {
    let mut list: *mut CSocketFile = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { lsock_files(&raw mut list, &raw mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut retval = HashMap::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_files = std::slice::from_raw_parts(list, nentries);

            for c_file in c_files {
                retval.insert(c_file.kvaddr, c_file.pid);
            }

            libc::free(list.cast::<libc::c_void>());
        }
    }

    Ok(retval)
}

pub(super) fn get_process_name(pid: i32) -> io::Result<String> {
    unsafe {
        let name_ptr = proc_name(pid);
        if name_ptr.is_null() {
            return Err(io::Error::last_os_error());
        }

        let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();

        libc::free(name_ptr.cast::<libc::c_void>());

        Ok(name)
    }
}

pub(super) fn get_process_path(pid: i32) -> io::Result<String> {
    unsafe {
        let path_ptr = proc_path(pid);
        if path_ptr.is_null() {
            return Err(io::Error::last_os_error());
        }

        let path = CStr::from_ptr(path_ptr).to_string_lossy().into_owned();

        libc::free(path_ptr.cast::<libc::c_void>());

        Ok(path)
    }
}

fn get_process_name_path(pid: i32) -> Option<(String, String)> {
    let name = get_process_name(pid).ok();
    let path = Some(get_process_path(pid).unwrap_or_default());

    name.zip(path)
}

pub(super) struct ProcNamesPathsCache {
    cache: HashMap<i32, Option<(String, String)>>,
}

impl ProcNamesPathsCache {
    pub(super) fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub(super) fn get(&mut self, pid: i32) -> Option<(String, String)> {
        if let Entry::Vacant(e) = self.cache.entry(pid) {
            e.insert(get_process_name_path(pid));
        }

        self.cache.get(&pid).cloned().flatten()
    }
}
