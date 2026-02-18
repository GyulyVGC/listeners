use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use crate::Protocol;

#[cfg(target_os = "freebsd")]
type KvAddr = usize;
#[cfg(target_os = "netbsd")]
type KvAddr = u64;

#[cfg(target_os = "freebsd")]
pub mod freebsd;
#[cfg(target_os = "netbsd")]
pub mod netbsd;
#[cfg(target_os = "openbsd")]
pub mod openbsd;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct SocketInfo {
    pub(super) address: SocketAddr,
    pub(super) protocol: Protocol,
    #[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
    pub(super) kvaddr: KvAddr,
}

#[repr(C)]
pub(super) union CAddress {
    ipv4: [u8; 4],
    ipv6: [u8; 16],
}

#[repr(C)]
pub(super) struct CSocketAddress {
    addr: CAddress,
    family: i32,
}

#[repr(C)]
pub(super) struct CSocketInfo {
    address: CSocketAddress,
    #[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
    kvaddr: KvAddr,
    protocol: i32,
    port: u16,
}

impl From<&CSocketInfo> for SocketInfo {
    fn from(value: &CSocketInfo) -> Self {
        let c_sock_addr = &value.address;
        let ip = if c_sock_addr.family == libc::AF_INET {
            let octets = unsafe { c_sock_addr.addr.ipv4 };
            IpAddr::V4(Ipv4Addr::from(octets))
        } else {
            let octets = unsafe { c_sock_addr.addr.ipv6 };
            IpAddr::V6(Ipv6Addr::from(octets))
        };

        SocketInfo {
            #[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
            kvaddr: value.kvaddr,
            address: SocketAddr::new(ip, value.port),
            protocol: match value.protocol {
                libc::IPPROTO_TCP => Protocol::TCP,
                _ => Protocol::UDP,
            },
        }
    }
}
