use crate::Listener;
use crate::Protocol;
use crate::SocketState;
use crate::platform::windows::socket_table::SocketTable;
use crate::platform::windows::tcp_table::TcpTable;
use crate::platform::windows::tcp6_table::Tcp6Table;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ffi::CStr;
use std::mem::size_of;
use std::mem::zeroed;
use std::net::{IpAddr, SocketAddr};
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use windows_sys::Win32::Foundation::{CloseHandle, FALSE, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First, Process32Next, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
};

use super::udp_table::UdpTable;
use super::udp6_table::Udp6Table;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct ProtoListener {
    local_addr: IpAddr,
    local_port: u16,
    pub(super) pid: u32,
    protocol: Protocol,
    state: SocketState,
}

impl ProtoListener {
    pub(super) fn get_all() -> Vec<ProtoListener> {
        Self::table_entries::<TcpTable>()
            .into_iter()
            .flatten()
            .chain(Self::table_entries::<Tcp6Table>().into_iter().flatten())
            .chain(Self::table_entries::<UdpTable>().into_iter().flatten())
            .chain(Self::table_entries::<Udp6Table>().into_iter().flatten())
            .collect()
    }

    pub(super) fn get_by_port(port: u16, protocol: Protocol) -> crate::Result<ProtoListener> {
        match protocol {
            Protocol::TCP => Self::table_entry_by_port::<TcpTable>(port)
                .or_else(|_| Self::table_entry_by_port::<Tcp6Table>(port)),
            Protocol::UDP => Self::table_entry_by_port::<UdpTable>(port)
                .or_else(|_| Self::table_entry_by_port::<Udp6Table>(port)),
        }
    }

    fn table_entries<Table: SocketTable>() -> crate::Result<Vec<Self>> {
        let mut proto_listeners = Vec::new();
        let table = Table::get_table()?;
        let rows_count = Table::get_rows_count(&table);
        for i in 0..rows_count {
            if let Some(proto_listener) = Table::get_proto_listener(&table, i, None) {
                proto_listeners.push(proto_listener);
            }
        }
        Ok(proto_listeners)
    }

    fn table_entry_by_port<Table: SocketTable>(port: u16) -> crate::Result<Self> {
        let table = Table::get_table()?;
        let rows_count = Table::get_rows_count(&table);
        for i in 0..rows_count {
            if let Some(proto_listener) = Table::get_proto_listener(&table, i, Some(port)) {
                return Ok(proto_listener);
            }
        }
        Err("No listener found on port".into())
    }

    pub(super) fn new(
        local_addr: IpAddr,
        local_port: u16,
        pid: u32,
        protocol: Protocol,
        state: SocketState,
    ) -> Self {
        Self {
            local_addr,
            local_port,
            pid,
            protocol,
            state,
        }
    }
}

pub(super) fn pname_ppath(pid: u32) -> Option<(String, String)> {
    let path_str = ppath(pid);

    let path = Path::new(&path_str);
    if path.is_file()
        && let Some(name) = path.file_name()
        && !name.is_empty()
    {
        let name_str = name.to_string_lossy().into_owned();
        return Some((name_str, path_str));
    }

    let pname = pname(pid);
    pname.zip(Some(path_str))
}

pub(super) struct PidNamePathCache {
    cache: HashMap<u32, Option<(String, String)>>,
    names: HashMap<u32, String>,
}

impl PidNamePathCache {
    pub(super) fn new() -> Self {
        Self {
            names: pname_collect(),
            cache: HashMap::new(),
        }
    }

    pub(super) fn get(&mut self, proto_listener: ProtoListener) -> Option<Listener> {
        let pid = proto_listener.pid;

        if let Entry::Vacant(e) = self.cache.entry(pid) {
            let name = self.names.get(&pid).cloned();
            let path = ppath(pid);
            e.insert(name.zip(Some(path)));
        }

        self.cache
            .get(&pid)
            .cloned()
            .flatten()
            .map(|(pname, ppath)| {
                let socket = SocketAddr::new(proto_listener.local_addr, proto_listener.local_port);
                Listener::new(
                    pid,
                    pname,
                    ppath,
                    socket,
                    proto_listener.protocol,
                    proto_listener.state,
                )
            })
    }
}

fn is_invalid(handle: HANDLE) -> bool {
    handle.is_null() || handle == INVALID_HANDLE_VALUE
}

/// Takes a snapshot of the running processes.
fn process_snapshot() -> Option<HANDLE> {
    let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    (!is_invalid(handle)).then_some(handle)
}

/// Reads the `szExeFile` field of a process entry.
///
/// Returns `None` if the field isn't NUL-terminated or isn't valid UTF-8.
fn exe_file(process: &PROCESSENTRY32) -> Option<String> {
    let raw = &process.szExeFile;
    // SAFETY: `c_char` and `u8` share their layout, and the length is taken from the
    // array itself, so the read stays within `szExeFile` even if the OS didn't
    // NUL-terminate it.
    let bytes = unsafe { std::slice::from_raw_parts(raw.as_ptr().cast::<u8>(), raw.len()) };
    let name = CStr::from_bytes_until_nul(bytes).ok()?;
    name.to_str().ok().map(str::to_owned)
}

fn pname(pid: u32) -> Option<String> {
    let dw_size = u32::try_from(size_of::<PROCESSENTRY32>()).ok()?;
    let h = process_snapshot()?;

    let mut process = unsafe { zeroed::<PROCESSENTRY32>() };
    process.dwSize = dw_size;

    let mut result = None;

    if unsafe { Process32First(h, &raw mut process) } != FALSE {
        loop {
            if process.th32ProcessID == pid {
                result = exe_file(&process);
                break;
            }

            if unsafe { Process32Next(h, &raw mut process) } == FALSE {
                break;
            }
        }
    }

    unsafe {
        let _ = CloseHandle(h);
    }

    result
}

fn ppath(pid: u32) -> String {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);
        if is_invalid(handle) {
            return String::new();
        }

        let mut buffer: [u16; 1024] = [0; 1024];
        let mut size = u32::try_from(buffer.len()).unwrap_or_default();

        let result = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            buffer.as_mut_ptr(),
            &raw mut size,
        );
        let _ = CloseHandle(handle);

        if result == FALSE {
            return String::new();
        }

        let path = std::ffi::OsString::from_wide(&buffer[..size as usize]);
        path.to_string_lossy().into_owned()
    }
}

fn pname_collect() -> HashMap<u32, String> {
    let mut ret_val = HashMap::default();

    let Ok(dw_size) = u32::try_from(size_of::<PROCESSENTRY32>()) else {
        return ret_val;
    };
    let Some(h) = process_snapshot() else {
        return ret_val;
    };

    let mut process = unsafe { zeroed::<PROCESSENTRY32>() };
    process.dwSize = dw_size;

    if unsafe { Process32First(h, &raw mut process) } != FALSE {
        loop {
            if let Some(name) = exe_file(&process) {
                let id = process.th32ProcessID;
                ret_val.insert(id, name);
            }

            if unsafe { Process32Next(h, &raw mut process) } == FALSE {
                break;
            }
        }
    }

    unsafe {
        let _ = CloseHandle(h);
    };

    ret_val
}
