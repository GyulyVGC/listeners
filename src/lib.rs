use std::fmt::Display;
use std::net::{IpAddr, SocketAddr};

#[cfg(not(target_os = "windows"))]
use libproc::libproc::proc_pid;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

/// A struct representing a process that is listening on a socket
pub struct Listener {
    /// The process ID of the listener process
    pid: u32,
    /// The name of the listener process
    pname: String,
    /// The local socket this listener is listening on
    socket: SocketAddr,
}

impl Listener {
    #[must_use]
    pub fn pid(&self) -> u32 {
        self.pid
    }

    #[must_use]
    pub fn pname(&self) -> &str {
        &self.pname
    }

    #[must_use]
    pub fn socket(&self) -> &SocketAddr {
        &self.socket
    }
}

impl Display for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PID: {:<10} Process name: {:<20} Socket: {:<26}",
            self.pid, self.pname, self.socket
        )
    }
}

#[must_use]
pub fn get_all_listeners() -> Vec<Listener> {
    let mut listeners = Vec::new();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap_or_default();

    let mut add_listeners = |pids: Vec<u32>, ip: IpAddr, port: u16| {
        for pid in pids {
            if let Some(pname) = get_name_from_pid(pid) {
                listeners.push(Listener {
                    pid,
                    pname,
                    socket: SocketAddr::new(ip, port),
                });
            }
        }
    };

    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                add_listeners(si.associated_pids, tcp_si.local_addr, tcp_si.local_port);
            }
            ProtocolSocketInfo::Udp(udp_si) => {
                add_listeners(si.associated_pids, udp_si.local_addr, udp_si.local_port);
            }
        }
    }

    listeners
}

#[cfg(target_os = "windows")]
unsafe fn get_name_from_pid(pid: u32) -> Option<String> {
    use std::mem::size_of;
    use std::mem::zeroed;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    };

    let h = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();

    let mut process = zeroed::<PROCESSENTRY32>();
    process.dwSize = size_of::<PROCESSENTRY32>() as u32;

    if Process32First(h, &mut process).as_bool() {
        loop {
            if Process32Next(h, &mut process).as_bool() {
                let id: u32 = process.th32ProcessID;
                if id == pid {
                    break;
                }
            } else {
                return None;
            }
        }
    }

    CloseHandle(h);

    let name = process.szExeFile;
    let mut temp: Vec<u8> = vec![];
    let len = name
        .iter()
        .position(|&x| x == windows::Win32::Foundation::CHAR(0))
        .unwrap();

    for i in name.iter() {
        temp.push(i.0.clone());
    }
    Some(String::from_utf8(temp[0..len].to_vec()).unwrap_or_default())
}

#[cfg(not(target_os = "windows"))]
fn get_name_from_pid(pid: u32) -> Option<String> {
    #[allow(clippy::cast_possible_wrap)]
    proc_pid::name(pid as i32).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
}
