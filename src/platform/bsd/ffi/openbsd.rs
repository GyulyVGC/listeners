use super::{CSocketInfo, SocketInfo};
use libc::{KI_MAXCOMLEN, pid_t};
use std::{
    collections::HashSet,
    ffi::CStr,
    io,
    os::raw::{c_char, c_int},
    ptr,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(in crate::platform::bsd) struct ProcessInfo {
    pub(in crate::platform::bsd) pid: i32,
    pub(in crate::platform::bsd) name: String,
}

#[repr(C)]
struct CProcessInfo {
    name: [c_char; KI_MAXCOMLEN as usize],
    pid: pid_t,
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
    fn openbsd_fetch_processes(list: *mut *mut CProcessInfo, nentries: *mut usize) -> c_int;
    fn openbsd_fetch_sockets_by_pid(
        pid: libc::pid_t,
        list: *mut *mut CSocketInfo,
        nentries: *mut usize,
    ) -> c_int;
}

pub(in crate::platform::bsd) fn get_all_processes() -> io::Result<HashSet<ProcessInfo>> {
    let mut list: *mut CProcessInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { openbsd_fetch_processes(&raw mut list, &raw mut nentries) } != 0 {
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

pub(in crate::platform::bsd) fn get_sockets(pid: i32) -> io::Result<Vec<SocketInfo>> {
    let mut list: *mut CSocketInfo = ptr::null_mut();
    let mut nentries: usize = 0;

    if unsafe { openbsd_fetch_sockets_by_pid(pid, &raw mut list, &raw mut nentries) } != 0 {
        return Err(io::Error::last_os_error());
    }

    let mut sockets = Vec::new();

    if nentries > 0 && !list.is_null() {
        unsafe {
            let c_sockets = std::slice::from_raw_parts(list, nentries);

            for c_socket in c_sockets {
                sockets.push(c_socket.into());
            }

            libc::free(list.cast::<libc::c_void>());
        }
    }

    Ok(sockets)
}
