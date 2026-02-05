use libc;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use super::ffi::{CSocketAddress, CSocketInfo};
use crate::Protocol;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SocketInfo {
    pub address: SocketAddr,
    pub protocol: Protocol,
}

impl From<&CSocketInfo> for SocketInfo {
    fn from(socket: &CSocketInfo) -> Self {
        Self {
            address: to_sockaddr(&socket.address, socket.port),
            protocol: match socket.protocol {
                0 => Protocol::TCP,
                _ => Protocol::UDP,
            },
        }
    }
}

fn to_sockaddr(address: &CSocketAddress, port: u16) -> SocketAddr {
    if address.family == libc::AF_INET {
        let address = unsafe {
            Ipv4Addr::new(
                address.addr.ipv4[0],
                address.addr.ipv4[1],
                address.addr.ipv4[2],
                address.addr.ipv4[3],
            )
        };

        SocketAddr::V4(SocketAddrV4::new(address, port))
    } else {
        let address = unsafe {
            Ipv6Addr::new(
                u16::from_le_bytes([address.addr.ipv6[0], address.addr.ipv6[1]]),
                u16::from_le_bytes([address.addr.ipv6[2], address.addr.ipv6[3]]),
                u16::from_le_bytes([address.addr.ipv6[4], address.addr.ipv6[5]]),
                u16::from_le_bytes([address.addr.ipv6[6], address.addr.ipv6[7]]),
                u16::from_le_bytes([address.addr.ipv6[8], address.addr.ipv6[9]]),
                u16::from_le_bytes([address.addr.ipv6[10], address.addr.ipv6[11]]),
                u16::from_le_bytes([address.addr.ipv6[12], address.addr.ipv6[13]]),
                u16::from_le_bytes([address.addr.ipv6[14], address.addr.ipv6[15]]),
            )
        };

        SocketAddr::V6(SocketAddrV6::new(address, port, 0, 0))
    }
}
