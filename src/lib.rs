use std::fmt::Display;
use std::net::SocketAddr;

pub use platform::get_all;

// use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState};

mod platform;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A struct representing a process that is listening on a socket
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Listener {
    /// The process ID of the listener process
    pid: u32,
    /// The name of the listener process
    pname: String,
    /// The local socket this listener is listening on
    socket: SocketAddr,
}

impl Listener {
    fn new(pid: u32, pname: String, socket: SocketAddr) -> Self {
        Self { pid, pname, socket }
    }

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
            "PID: {:<10} Process name: {:<25} Socket: {:<25}",
            self.pid, self.pname, self.socket
        )
    }
}

// #[must_use]
// pub fn get_all() -> Vec<Listener> {
//     get_with_filters(
//         AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
//         ProtocolFlags::TCP | ProtocolFlags::UDP,
//         None,
//         None,
//     )
// }
//
// #[must_use]
// pub fn get_for_nullnet(ip_addr: IpAddr) -> HashSet<String> {
//     get_with_filters(
//         AddressFamilyFlags::IPV4,
//         ProtocolFlags::TCP,
//         Some(TcpState::Listen),
//         Some(ip_addr),
//     )
//     .iter()
//     .map(|l| l.pname.clone())
//     .collect()
// }
//
// fn get_with_filters(
//     af_flags: AddressFamilyFlags,
//     proto_flags: ProtocolFlags,
//     tcp_state: Option<TcpState>,
//     ip_addr: Option<IpAddr>,
// ) -> Vec<Listener> {
//     let mut listeners = Vec::new();
//
//     let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap_or_default();
//
//     let mut add_listeners = |pids: Vec<u32>, ip: IpAddr, port: u16| {
//         for pid in pids {
//             if let Some(pname) = get_name_from_pid(pid) {
//                 if ip.is_unspecified() || ip_addr.is_none() || ip_addr.unwrap() == ip {
//                     listeners.push(Listener {
//                         pid,
//                         pname,
//                         socket: SocketAddr::new(ip, port),
//                     });
//                 }
//             }
//         }
//     };
//
//     for si in sockets_info {
//         match si.protocol_socket_info {
//             ProtocolSocketInfo::Tcp(tcp_si) => {
//                 if tcp_state.is_none() || tcp_si.state == tcp_state.unwrap() {
//                     add_listeners(si.associated_pids, tcp_si.local_addr, tcp_si.local_port);
//                 }
//             }
//             ProtocolSocketInfo::Udp(udp_si) => {
//                 add_listeners(si.associated_pids, udp_si.local_addr, udp_si.local_port);
//             }
//         }
//     }
//
//     listeners
// }
