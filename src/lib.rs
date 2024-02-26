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
            #[allow(clippy::cast_possible_wrap)]
            let pname = proc_pid::name(pid as i32).unwrap_or_default();
            listeners.push(Listener {
                pid,
                pname,
                socket: SocketAddr::new(ip, port),
            });
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

#[cfg(test)]
mod tests {
    use super::*;
}
