use super::ffi::CSocketInfo;
use crate::Protocol;
use std::net::SocketAddr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct SocketInfo {
    pub(super) address: SocketAddr,
    pub(super) protocol: Protocol,
}

// impl SocketInfo {
//     pub(super) fn get_all_listening() -> Vec<Self> {
//         get_listening_sockets_tcp()
//             .into_iter()
//             .chain(get_listening_sockets_tcp6())
//             .chain(get_listening_sockets_udp())
//             .chain(get_listening_sockets_udp6())
//             .collect()
//     }
//
//     pub(super) fn get_listening_on_port(port: u16, protocol: Protocol) -> Option<Self> {
//         match protocol {
//             Protocol::TCP => get_listening_sockets_tcp()
//                 .into_iter()
//                 .chain(get_listening_sockets_tcp6())
//                 .find(|s| s.address.port() == port),
//             Protocol::UDP => get_listening_sockets_udp()
//                 .into_iter()
//                 .chain(get_listening_sockets_udp6())
//                 .find(|s| s.address.port() == port),
//         }
//     }
// }

impl From<&CSocketInfo> for SocketInfo {
    fn from(socket: &CSocketInfo) -> Self {
        Self {
            address: socket.to_sockaddr(),
            protocol: match socket.protocol {
                0 => Protocol::TCP,
                _ => Protocol::UDP,
            },
        }
    }
}
