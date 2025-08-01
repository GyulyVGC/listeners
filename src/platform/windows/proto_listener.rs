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

#[derive(Debug)]
pub(super) struct ProtoListener {
    local_addr: IpAddr,
    local_port: u16,
    pid: u32,
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

    fn table_entries<Table: SocketTable>() -> crate::Result<Vec<Self>> {
        let mut proto_listeners = Vec::new();
        let table = Table::get_table()?;
        for i in 0..Table::get_rows_count(&table) {
            if let Some(proto_listener) = Table::get_proto_listener(&table, i) {
                proto_listeners.push(proto_listener);
            }
        }
        Ok(proto_listeners)
    }

    pub(super) fn new(local_addr: IpAddr, local_port: u16, pid: u32, protocol: Protocol) -> Self {
        Self {
            local_addr,
            local_port,
            pid,
            protocol,
        }
    }

    fn pname(&self) -> Option<String> {
        let pid = self.pid;

        let h = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()? };

        let mut process = unsafe { zeroed::<PROCESSENTRY32>() };
        process.dwSize = u32::try_from(size_of::<PROCESSENTRY32>()).ok()?;

        if unsafe { Process32First(h, &mut process) }.is_ok() {
            loop {
                if unsafe { Process32Next(h, &mut process) }.is_ok() {
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

        String::from_utf8(name[0..len].iter().map(|e| *e as u8).collect()).ok()
    }

    fn ppath(&self) -> String {
        let pid = self.pid;

        unsafe {
            let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
                return String::new();
            };
            if handle.is_invalid() {
                return String::new();
            }

            let mut buffer: [u16; 1024] = [0; 1024];
            let mut size = buffer.len() as u32;

            let _ = QueryFullProcessImageNameW(
                handle,
                PROCESS_NAME_FORMAT(0),
                PWSTR(buffer.as_mut_ptr()),
                &mut size,
            );
            let _ = CloseHandle(handle);

            let path = std::ffi::OsString::from_wide(&buffer[..size as usize]);
            path.to_string_lossy().into_owned()
        }
    }

    pub(super) fn to_listener(&self) -> Option<Listener> {
        let socket = SocketAddr::new(self.local_addr, self.local_port);
        let pname = self.pname()?;
        let ppath = self.ppath();
        Some(Listener::new(self.pid, pname, ppath, socket, self.protocol))
    }
}
