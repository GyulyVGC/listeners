use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::{io, ptr};

use super::socket_info::SocketInfo;
use crate::Process;

#[repr(C)]
pub(super) struct CSocketAddress {
    pub addr: CAddress,
    pub family: i32,
}

#[repr(C)]
pub(super) union CAddress {
    pub ipv4: [u8; 4],
    pub ipv6: [u8; 16],
}

#[repr(C)]
pub(super) struct CSocketInfo {
    pub address: CSocketAddress,
    pub port: u16,
    pub protocol: u32,
}

#[repr(C)]
pub(super) struct CProcessInfo {
    pub path: [c_char; libc::PATH_MAX as usize],
    pub name: [c_char; libc::COMMLEN + 1],
    pub pid: c_int,
}

unsafe extern "C" {
    fn lsock_tcp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_tcp6(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_udp(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn lsock_udp6(list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
    fn proc_list(list: *mut *mut CProcessInfo, nentries: *mut usize) -> c_int;
    fn proc_sockets(pid: c_int, list: *mut *mut CSocketInfo, nentries: *mut usize) -> c_int;
}

pub fn get_listening_sockets_tcp() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    let ret = unsafe { lsock_tcp(&mut list, &mut nentries) };

    if ret != 0 {
        return Err(io::Error::last_os_error());
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

    Ok(sockets)
}

pub fn get_listening_sockets_tcp6() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    let ret = unsafe { lsock_tcp6(&mut list, &mut nentries) };

    if ret != 0 {
        return Err(io::Error::last_os_error());
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

    Ok(sockets)
}

pub fn get_listening_sockets_udp() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    let ret = unsafe { lsock_udp(&mut list, &mut nentries) };

    if ret != 0 {
        return Err(io::Error::last_os_error());
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

    Ok(sockets)
}

pub fn get_listening_sockets_udp6() -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    let ret = unsafe { lsock_udp6(&mut list, &mut nentries) };

    if ret != 0 {
        return Err(io::Error::last_os_error());
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

    Ok(sockets)
}

pub fn get_processes() -> io::Result<Vec<Process>> {
    let mut list: *mut CProcessInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    let ret = unsafe { proc_list(&mut list, &mut nentries) };

    if ret != 0 {
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

pub fn get_process_all_sockets(pid: u32) -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    let ret = unsafe { proc_sockets(pid as c_int, &mut list, &mut nentries) };

    if ret != 0 {
        return Err(io::Error::last_os_error());
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

    Ok(sockets)
}
