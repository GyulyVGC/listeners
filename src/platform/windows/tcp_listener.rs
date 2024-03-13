use std::net::{IpAddr, SocketAddr};
use crate::Listener;

#[derive(Debug)]
pub(super) struct TcpListener {
    local_addr: IpAddr,
    local_port: u16,
    pid: u32,
}

impl TcpListener {
    pub(super) fn new(local_addr: IpAddr, local_port: u16, pid: u32) -> Self {
        Self {
            local_addr,
            local_port,
            pid,
        }
    }

    pub(super) fn local_addr(&self) -> IpAddr {
        self.local_addr
    }

    pub(super) fn local_port(&self) -> u16 {
        self.local_port
    }

    pub(super) fn pid(&self) -> u32 {
        self.pid
    }

    pub(super) fn pname(&self) -> Option<String> {
        use std::mem::size_of;
        use std::mem::zeroed;
        use windows::Win32::Foundation::CloseHandle;
        use windows::Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
        };

        let pid = self.pid;

        let h = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap() };

        let mut process = unsafe { zeroed::<PROCESSENTRY32>() };
        process.dwSize = size_of::<PROCESSENTRY32>() as u32;

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

        unsafe { CloseHandle(h).unwrap() };

        let name = process.szExeFile;
        let mut temp: Vec<u8> = vec![];
        let len = name.iter().position(|&x| x == 0).unwrap();

        for i in name.iter() {
            temp.push(*i as u8);
        }
        Some(String::from_utf8(temp[0..len].to_vec()).unwrap_or_default())
    }

    pub(super) fn to_listener(&self) -> Option<Listener> {
        let socket = SocketAddr::new(self.local_addr, self.local_port);
        let pname = self.pname()?;
        Some(Listener::new(self.pid, pname, socket))
    }
}
