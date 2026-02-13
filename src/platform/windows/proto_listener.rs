use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::mem::size_of;
use std::mem::zeroed;
use std::net::{IpAddr, SocketAddr};
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First, Process32Next, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
};
use windows::core::PWSTR;

use crate::Listener;
use crate::Protocol;
use crate::platform::windows::socket_table::SocketTable;
use crate::platform::windows::tcp_table::TcpTable;
use crate::platform::windows::tcp6_table::Tcp6Table;

use super::udp_table::UdpTable;
use super::udp6_table::Udp6Table;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct ProtoListener {
    local_addr: IpAddr,
    local_port: u16,
    pub(super) pid: u32,
    protocol: Protocol,
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

    pub(super) fn new(local_addr: IpAddr, local_port: u16, pid: u32, protocol: Protocol) -> Self {
        Self {
            local_addr,
            local_port,
            pid,
            protocol,
        }
    }
}

pub(super) fn pname_ppath(pid: u32) -> Option<(String, String)> {
    let pname = pname(pid);
    let ppath = Some(ppath(pid));
    pname.zip(ppath)
}

pub(super) struct PidNamePathCache {
    cache: HashMap<u32, Option<(String, String)>>,
}

impl PidNamePathCache {
    pub(super) fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub(super) fn get(&mut self, proto_listener: ProtoListener) -> Option<Listener> {
        let pid = proto_listener.pid;

        if let Entry::Vacant(e) = self.cache.entry(pid) {
            e.insert(pname_ppath(pid));
        }

        self.cache
            .get(&pid)
            .cloned()
            .flatten()
            .map(|(pname, ppath)| {
                let socket = SocketAddr::new(proto_listener.local_addr, proto_listener.local_port);
                Listener::new(pid, pname, ppath, socket, proto_listener.protocol)
            })
    }
}

fn pname(pid: u32) -> Option<String> {
    let h = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()? };

    let mut process = unsafe { zeroed::<PROCESSENTRY32>() };
    process.dwSize = u32::try_from(size_of::<PROCESSENTRY32>()).ok()?;

    if unsafe { Process32First(h, &raw mut process) }.is_ok() {
        loop {
            if unsafe { Process32Next(h, &raw mut process) }.is_ok() {
                let id: u32 = process.th32ProcessID;
                if id == pid {
                    break;
                }
            } else {
                return None;
            }
        }
    }

    unsafe {
        let _ = CloseHandle(h);
    };

    let name = process.szExeFile;
    let len = name.iter().position(|&x| x == 0)?;

    #[allow(clippy::cast_sign_loss)]
    String::from_utf8(name[0..len].iter().map(|e| *e as u8).collect()).ok()
}

fn ppath(pid: u32) -> String {
    unsafe {
        let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
            return String::new();
        };
        if handle.is_invalid() {
            return String::new();
        }

        let mut buffer: [u16; 1024] = [0; 1024];
        let mut size = u32::try_from(buffer.len()).unwrap_or_default();

        let _ = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(buffer.as_mut_ptr()),
            &raw mut size,
        );
        let _ = CloseHandle(handle);

        let path = std::ffi::OsString::from_wide(&buffer[..size as usize]);
        path.to_string_lossy().into_owned()
    }
}
